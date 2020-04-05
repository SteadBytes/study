//! Network Time Protocol client
//!
//! *Basic* NTP v4 ([RFC 5905](https://tools.ietf.org/html/rfc5905))
//! implementation to facilitate checking the system time.
use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Timelike, Utc};
use std::io::Result;
use std::net::UdpSocket;
use std::time::Duration;

/// Total length of an NTP message in bytes (12 * 4 byte words)
const NTP_MESSAGE_LENGTH: usize = 48;
/// Seconds between the (era 0) NTP epoch (1 Jan 1900) and the UNIX epoch
/// (1 Jan 1970)
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;

// TODO: Make all of these configurable
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";
const UDP_RESPONSE_TIMEOUT: Duration = Duration::from_secs(1);
const NTP_PORT: u16 = 123;
const NTP_SERVERS: [&'static str; 7] = [
    "pool.ntp.org",
    "time.nist.gov",
    "time.apple.com",
    "time.euro.apple.com",
    "time.google.com",
    "time2.google.com",
    "time.windows.com",
];

/// Represents a timestamp in NTP Timestamp Format
///
/// See [RFC 5905 section 6, figure 3](https://tools.ietf.org/html/rfc5905#section-6)
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                            Seconds                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                            Fraction                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
/// This struct encapsulates the seconds and fractions fields as described in
/// the RFC:
/// > It includes a 32-bit unsigned seconds field spanning 136 years and a 32-bit
/// > fraction field resolving 232 picoseconds.
///
/// Note: 232 picosecond fraction -> 1 second = 2^32
///
/// Note: This **does not** handle the rollover caused by the limit of
/// the 32-bit seconds field being reached on February 7 2036 due to the
/// explicit use of era 0 as a base for conversion (see `NTP_TO_UNIX_SECONDS`).
/// - TODO: Handle non-0 eras correctly
#[derive(Default, Debug, Copy, Clone, PartialEq)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

// TODO: Implement these as TryFrom and return Err if not era 0?
impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        // Assumes era 0 - see note above
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        // 1 second = 2^32
        let sec_to_nanos = 2_f64.powi(32) / 1_000_000_000.0;
        let nanos = (ntp.fraction as f64) / sec_to_nanos;
        Utc.timestamp(secs, nanos as u32)
    }
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        // Assumes era 0 - see note above
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        // 1 second = 2^32
        let nano_to_secs = 2_f64.powi(32) / 1e9;
        let fraction = utc.nanosecond() as f64 * nano_to_secs;
        NTPTimestamp {
            // Safe (assuming era 0 only) as u32::max_value() < era 0 seconds
            seconds: secs as u32,
            fraction: fraction.round() as u32,
        }
    }
}

/// Timestamps present in an NTP packet.
///
/// Note: Not all timestamps are present here as this module does not parse
/// those values directly.
#[derive(Debug)]
enum Timestamp {
    Receive,
    Transmit,
}

/// Represents an NTP packet.
///
/// See [RFC 5905 section 7.3, figure 3](https://tools.ietf.org/html/rfc5905#section-6)
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |LI | VN  |Mode |    Stratum     |     Poll      |  Precision   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Root Delay                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Root Dispersion                       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          Reference ID                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                     Reference Timestamp (64)                  +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                      Origin Timestamp (64)                    +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                      Receive Timestamp (64)                   +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                      Transmit Timestamp (64)                  +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// .                                                               .
/// .                    Extension Field 1 (variable)               .
/// .                                                               .
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// .                                                               .
/// .                    Extension Field 2 (variable)               .
/// .                                                               .
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          Key Identifier                       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                            dgst (128)                         |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
struct NTPMessage {
    /// Bytes of an NTP packet.
    data: [u8; NTP_MESSAGE_LENGTH],
}

impl NTPMessage {
    const VERSION: u8 = 4;
    /// `Mode` value for a client request - see RFC 5905 figure 10
    const CLIENT_MODE: u8 = 3;

    /// Initialise an empty `NTPMessage`.
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }

    /// Initialise an `NTPMessage` for a client request.
    ///
    /// Note: Leap Indicator is *unset*.
    fn client() -> Self {
        let mut msg = NTPMessage::new();

        // Set vn and mode, leave li unset
        msg.data[0] |= NTPMessage::VERSION << 3;
        msg.data[0] |= NTPMessage::CLIENT_MODE;
        msg
    }

    /// Parse a specific timestamp from NTP packet data.
    fn parse_timestamp(&self, t: Timestamp) -> Result<NTPTimestamp> {
        let offset = match t {
            Timestamp::Receive => 32,
            Timestamp::Transmit => 40,
        };
        let mut reader = &self.data[offset..offset + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;
        println!("{:0x}", fraction);
        Ok(NTPTimestamp { seconds, fraction })
    }

    /// Received timestamp; time that the server received the client's message.
    pub fn rx_time(&self) -> Result<NTPTimestamp> {
        self.parse_timestamp(Timestamp::Receive)
    }

    /// Transmitted timestamp; time that the server sent reply.
    pub fn tx_time(&self) -> Result<NTPTimestamp> {
        self.parse_timestamp(Timestamp::Transmit)
    }
}

/// Result of an NTP roundtrip, containing `t1`, `t2`, `t3`, `t4` time values
/// for the request/response cycle.
///
/// https://tools.ietf.org/html/rfc5905#section-8
#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    /// Calculate the offset `θ` of the system clock as measured by this
    /// roundtrip result:
    ///
    /// `θ = ((t1 - t0) + (t2 - t3)) / 2`
    fn offset(&self) -> i64 {
        let duration = (self.t2 - self.t1) + (self.t3 - self.t4);
        duration.num_milliseconds() / 2
    }

    /// Calculate the network delay `δ` experienced during this roundtrip
    /// result:
    ///
    /// `δ = (t4 - t1) - (t3 - t2)`
    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}

/// Perform an NTP request/response roundtrip to `host:port` measuring values
/// for `t1`, `t2`, `t3`, `t4`. Returns `Err` if no response received from
/// `host` within `UDP_RESPONSE_TIMEOUT`.
// TODO: Dynamic server discovery https://tools.ietf.org/html/rfc5905#section-3.1
fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult> {
    // Note: t1 and t4 are measured *as soon as possible* before/after
    // sending/receiving UDP request/response to maximise accuracy.

    // Do all set up *prior* to measuring t1
    let destination = format!("{}:{}", host, port);
    // TODO: Configurable?
    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(&destination)
        .expect("unable to bind local UDP socket");
    udp.set_read_timeout(Some(UDP_RESPONSE_TIMEOUT))?;

    let request = NTPMessage::client();
    let mut response = NTPMessage::new();

    // 'Cheat' by not encoding t1 in outbound message to save some work as t1
    // (although specified in the NTP packet spec) is not used by dest server
    let t1 = Utc::now();

    // Send immediately after measuring t1
    udp.send(&request.data)?;

    // Block until data received (or timeout)
    udp.recv_from(&mut response.data)?;

    // Measure t4 immediately after receiving response
    let t4 = Utc::now();

    // Parse remaining timestamps from response
    let t2: DateTime<Utc> = response.rx_time().unwrap().into();
    let t3: DateTime<Utc> = response.tx_time().unwrap().into();

    Ok(NTPResult { t1, t2, t3, t4 })
}

/// Check the system time against NTP servers, returning the average offset.
///
/// Note: This is a simple implementation and does not include the [filtering](https://tools.ietf.org/html/rfc5905#section-10)
/// used by the reference implementation to reduce the effect of bad actors and
/// spurious results. Instead several simpler measures are taken:
///
/// - Slow servers are aggressively *penalised*
///   - Average offset is calculated as a mean of the differences, weighted by
///     `1 / θ^2`
/// - Check against known, trusted NTP servers
///   - See `NTP_SERVERS`
/// - Cap adjustments to system time to ensure no single NTP result has a large
///   effect.
pub fn check_time() -> Result<f64> {
    let mut results = Vec::with_capacity(NTP_SERVERS.len());

    for &server in NTP_SERVERS.iter() {
        print!("{} =>", server);

        let maybe_result = ntp_roundtrip(&server, NTP_PORT);

        match maybe_result {
            Ok(time) => {
                println!(" {}ms away from local time", time.offset());
                results.push(time);
            }
            Err(_) => println!(" ? [response took too long]"),
        };
    }

    // Need at least 3 NTP responses to calculate a reasonable average offset.
    // TODO: Document & implement custom error type
    if results.len() < 3 {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Insufficient number of servers returned responses",
        ))
    } else {
        Ok(average_offset(&results))
    }
}

// TODO: Test
fn average_offset(results: &[NTPResult]) -> f64 {
    let offset_weights = results.iter().filter_map(|r| {
        // Penalise slow response times
        let weight = 1000000.0 / (r.delay() as f64).powi(2);
        if weight.is_finite() {
            Some((r.offset() as f64, weight))
        } else {
            None
        }
    });
    weighted_mean(offset_weights)
}

// TODO: Test
/// Calculates the weighted mean from a sequence of `(value, weight)` pairs.
fn weighted_mean<I: Iterator<Item = (f64, f64)>>(weighted_vals: I) -> f64 {
    let (result, sum_of_weights) =
        weighted_vals.fold((0.0, 0.0), |acc, (v, w)| (acc.0 + v * w, acc.1 + w));
    result / sum_of_weights
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_datetime_from_ntp_timestamp() {
        let ts = NTPTimestamp {
            seconds: 3314714339,
            fraction: 51539608,
        };

        let t: DateTime<Utc> = ts.into();

        assert_eq!(
            t,
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd(2005, 1, 14).and_hms_milli(17, 58, 59, 12),
                Utc,
            )
        );
    }

    #[test]
    fn test_ntp_timestamp_from_datetime() {
        let t = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd(2005, 1, 14).and_hms_milli(17, 58, 59, 12),
            Utc,
        );

        let ts: NTPTimestamp = t.into();

        assert_eq!(
            ts,
            NTPTimestamp {
                seconds: 3314714339,
                fraction: 51539608,
            }
        );
    }

    // TODO: Property-based tests?
    #[test]
    fn test_datetime_from_ntp_timestamp_roundtrip() {
        let ts = NTPTimestamp {
            seconds: 3314714339,
            fraction: 51539608,
        };

        let t: DateTime<Utc> = ts.into();
        let ts_: NTPTimestamp = t.into();

        assert_eq!(ts, ts_);
    }

    #[test]
    fn test_ntp_timestamp_from_datetime_roundtrip() {
        let t = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd(2005, 1, 14).and_hms_milli(17, 58, 59, 12),
            Utc,
        );

        let ts: NTPTimestamp = t.into();
        let t_: DateTime<Utc> = ts.into();

        assert_eq!(t, t_);
    }

    #[test]
    #[ignore] // See TODO above about eras
    fn test_ntp_timestamp_from_datetime_roundtrip_era_1() {
        let t = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd(2038, 1, 14).and_hms_milli(17, 58, 59, 12),
            Utc,
        );

        let ts: NTPTimestamp = t.into();
        let t_: DateTime<Utc> = ts.into();

        assert_eq!(t, t_);
    }

    #[test]
    fn test_ntp_result() {
        // Captured from wireshark after running ntpdate -q ntp.ubuntu.com:
        //
        // Frame 5: 90 bytes on wire (720 bits), 90 bytes captured (720 bits) on interface 0
        // Ethernet II, Src: 7a:8a:20:f6:3c:2a (7a:8a:20:f6:3c:2a), Dst: Giga-Byt_2d:a6:19 (e0:d5:5e:2d:a6:19)
        // Internet Protocol Version 4, Src: 91.189.91.157, Dst: 192.168.140.170
        // User Datagram Protocol, Src Port: 123, Dst Port: 38133
        // Network Time Protocol (NTP Version 4, server)
        //     Flags: 0x24, Leap Indicator: no warning, Version number: NTP Version 4, Mode: server
        //         00.. .... = Leap Indicator: no warning (0)
        //         ..10 0... = Version number: NTP Version 4 (4)
        //         .... .100 = Mode: server (4)
        //     [Request In: 4]
        //     [Delta Time: 0.106602338 seconds]
        //     Peer Clock Stratum: secondary reference (2)
        //     Peer Polling Interval: invalid (3)
        //     Peer Clock Precision: 0.000000 seconds
        //     Root Delay: 0.047806 seconds
        //     Root Dispersion: 0.031921 seconds
        //     Reference ID: 132.163.96.1
        //     Reference Timestamp: Apr  3, 2020 05:46:17.765617804 UTC
        //     Origin Timestamp: Apr  3, 2020 06:04:12.815443824 UTC
        //     Receive Timestamp: Apr  3, 2020 06:04:12.873512707 UTC
        //     Transmit Timestamp: Apr  3, 2020 06:04:12.873535881 UTC
        //
        // Copied from Wireshark "Show packet bytes" as C Array
        let data = [
            0x24, 0x02, 0x03, 0xe8, 0x00, 0x00, 0x0c, 0x3d, 0x00, 0x00, 0x08, 0x2c, 0x84, 0xa3,
            0x60, 0x01, 0xe2, 0x31, 0x4c, 0x29, 0xc3, 0xff, 0x87, 0x47, 0xe2, 0x31, 0x50, 0x5c,
            0xd0, 0xc0, 0xed, 0x30, 0xe2, 0x31, 0x50, 0x5c, 0xdf, 0x9e, 0x87, 0x5f, 0xe2, 0x31,
            0x50, 0x5c, 0xdf, 0xa0, 0x0c, 0x2b,
        ];

        let msg = NTPMessage { data };

        // 64 bits from byte offset 40 0xe231505cdf9e875f
        let tx = msg.tx_time().unwrap();
        assert_eq!(
            tx,
            NTPTimestamp {
                seconds: 3794882652,  // 0xe231505c
                fraction: 3751808043  // 0xdf9e875f
            }
        );

        // 64 bits from byte offset 32 0xe231505cdf9e875f
        let tx = msg.rx_time().unwrap();
        assert_eq!(
            tx,
            NTPTimestamp {
                seconds: 3794882652,  // 0xe231505c
                fraction: 3751708511  // 0xdf9e875f
            }
        );
    }
}
