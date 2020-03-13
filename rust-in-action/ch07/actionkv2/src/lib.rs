#[macro_use]
extern crate serde_derive;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::path::Path;

/// Data that tends to be used a string but is in raw bytes form
pub type ByteString = Vec<u8>;
pub type ByteStr = [u8];

///
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

/// In-memory Key-Value store.
///
/// Data is stored on disk using the Bitcask log-structured hash table format
/// in an **append-only** fashion.
#[derive(Debug)]
pub struct ActionKV {
    f: File,
    /// Mapping between keys and file locations
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    /// Initialise an empty `ActionKV` backed by store file at `path`. The file
    /// contents are *not* loaded into memory - see [`load`]: #method.load.
    pub fn open(path: &Path) -> io::Result<ActionKV> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

        Ok(ActionKV {
            f,
            index: HashMap::new(),
        })
    }

    /// Create an in-memory index from the contents of the store file.
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        // Load `KeyValuePair`s from `f` until EOF - more recent updates will
        // overwrite stale data as writing records is *append-only*
        loop {
            // Number of bytes from the start of the file
            let current_position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        // Expected error - all records read from file
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err),
                    }
                }
            };

            self.index.insert(kv.key, current_position);
        }

        Ok(())
    }

    /// Read a `KeyValuePair` record from `f` - assumes that `f` is at a valid
    /// start position for a record. This will advance the position of `f` by
    /// one record.
    ///
    /// # Panics
    ///
    /// Panics if data retrieved from `self.f` is corrupted.
    ///
    /// # Implementation Details
    ///
    /// A `KeyValuePair` is stored in the format defined by Bitcask:
    ///
    /// ```text
    /// Fixed-width header      Variable-length body
    ///  -----------------     --------------------------
    /// /                  \  /                          \
    ///
    /// +=====+=====+=====+====== - - +============= - - +
    /// | u32 | u32 | u32 | [u8]      | [u8]             |
    /// +=====+=====+=====+====== - - +============= - - +
    ///
    ///  ~~~~~ ~~~~~ ~~~~~  ~~~~~~~~  ~~~~~~~~~~~~~~
    ///    |     |     |        |            \
    ///    |     |     |        \           value (val_len bytes)
    ///    |     |     \       key (key_len bytes)
    ///    |     \    val_len (4 bytes)
    ///    \   key_len (4 bytes)
    ///    checksum (4 bytes)
    ///
    /// Algorithm:
    ///
    /// 1. Read header.
    /// 2. Use `key_len` and `val_len` sections to compute the body length.
    /// 3. Read the body.
    /// 4. Verify the body using the header `checksum`.
    /// 5. Split the body into `key` and `value`.
    /// ```
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        // Read & parse header secion
        let expected_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        // Read data section into buffer
        let mut data = ByteString::with_capacity(data_len as usize);

        // Use a reference within a short-lived block to avoid ownership issues
        // due to `take` creating a new `Read` value
        {
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }

        // Don't incur this overhead in optimised builds
        debug_assert_eq!(data.len(), data_len as usize);

        let actual_checksum = crc32::checksum_ieee(&data);
        if actual_checksum != expected_checksum {
            panic!(
                "data corruption encountered ({:08X} != {:08X})",
                actual_checksum, expected_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    /// Find the position and value of `key` if it exists.
    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);
        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    // Expected error - all records read from file
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            if kv.key == target {
                found = Some((position, kv.value));
            }

            // Append-only -> keep looping until EOF in case the key has been
            // overwritten (updated or deleted)
        }

        Ok(found)
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(p) => *p,
        };

        let kv = self.get_at(position)?;
        Ok(Some(ByteString::from(kv.value)))
    }

    /// Get a `KeyValuePair` from `self.f` at `position`.
    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        Ok(ActionKV::process_record(&mut f)?)
    }

    /// Insert `value` into index at `key`.
    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_to_file(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    /// Insert `key` and `value` into `self.f`, returning the position of the
    /// record in `self.f` (it's current position).
    ///
    /// - **Does not** update the in memory index `self.index`.
    ///
    /// Essentially the reverse of [`process_record`]: #method.process_record.
    ///
    /// Algorithm:
    ///
    /// 1. Join `key` and `value` to create the body.
    /// 2. Calculate the `checksum` of the body.
    /// 3. Seek to EOF.
    /// 4. Write header.
    /// 5. Write body.
    /// 6. Return record position.
    fn insert_to_file(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        // Use `BufWriter` to batch multiple short writes together to increase
        // throughput
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        // Join the key an value into a single value
        let mut tmp: ByteString = key.iter().chain(value).copied().collect();

        let checksum = crc32::checksum_ieee(&tmp);

        // Append-only insertion -> write new record at EOF
        let position = f.seek(SeekFrom::End(0))?;

        // Write header
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        // Write body
        f.write_all(&mut tmp)?;

        Ok(position)
    }

    // Append-only -> `insert` and `delete` are just variations of `insert`

    #[inline]
    /// Update `key` with `value`.
    ///
    /// Note: `key` *does not* have to be present for this to succeeed - a new
    /// key will be added.
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    /// Delete entry `key` from store.
    ///
    /// Note: `key` *does not* have to be present for this to succeeed.
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }
}
