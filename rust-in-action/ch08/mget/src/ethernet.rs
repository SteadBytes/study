//! Low level address and port represetnation/generation
//!

use rand::RngCore;
use smoltcp::wire;
use std::fmt;
use std::fmt::Display;
use std::fs::read_to_string;
use std::net::TcpListener;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(String),
    NoFreePort,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

fn is_tcp_port_free(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

/// Parse ephemeral port range from `/proc/sys/net/ipv4/ip_local_port_range`
fn get_ephemeral_port_range() -> Result<std::ops::Range<u16>, Error> {
    let s = read_to_string("/proc/sys/net/ipv4/ip_local_port_range").map_err(Error::IO)?;
    let mut parts = s.split_whitespace().map(|s| s.parse::<u16>());
    match (parts.next(), parts.next(), parts.next()) {
        (Some(Ok(lb)), Some(Ok(ub)), None) => Ok(lb..ub),
        _ => Err(Error::Parse(s)),
    }
}

/// Find a free ephemeral TCP port
///
/// It is advisable to bind the port returned **as soon as possible** as this
/// function only guarantees that the port is free when it executes. It is
/// possible (though unlikely) that another process will bind the port in
/// between this function returning and the calling code using it.
pub fn free_tcp_port() -> Result<u16, Error> {
    match get_ephemeral_port_range()?.find(|p| is_tcp_port_free(*p)) {
        Some(r) => Ok(r),
        None => Err(Error::NoFreePort),
    }
}

const LOCAL: u8 = 0b_0000_0010;
const UNICAST: u8 = 0b_0000_0001;

/// Representation of a MAC address according to IEEE 802.
///
/// # MAC Address Details
///
/// Media access control address is a *unique* identifier assigned to a NIC
/// to be used as a network address.
/// 2 forms:
/// - Universally administered (universal)
///   - Set when devices are manufactured
///   - Consist of:
///     - Prefix assigned by IEEE Registration Authority to the manufacturer
///     - Manufacturer-chosen scheme for the remaining bits
/// - Locally administered (local)
///   - Created by devices to set their own MAC address *without* registration
///
/// 2 modes:
/// - Unicast
///   - Transport information between two points *in direct contact*
///     - e.g. laptop <-> router
///   - Frames are accepted *only* by the NIC with the matching MAC address
/// - Multicast
///   - NICs choose whether to accept a frame based on criteria *other* than
///     matching MAC address
///
/// MAC addresses are 48 bits wide. The structure depends on whether the
/// address is unversal or local:
///
/// ```text
/// +--------+--------+--------+--------+--------+--------+
/// |||||||||||||||||||||||||||||||||||||||||||||||||||||||
/// +--------+--------+--------+--------+--------+--------+
///
/// |______..__________________|__________________________| Universal
///          Organisation                Device
///
/// |______..__________________|__________________________| Local
///                          Device
///
/// |......__..................|..........................| Common fields
///       Flags
///
/// |        |
/// |        +---------------
/// |                       |
/// v                       v
/// +-----------------------+
/// |b7|b6|b5|b4|b3|b2|b1|b0|
/// +-----------------------+
/// ```
///
/// The flags indicate unicast/multicast and universal/local:
/// - Group bit = unicast/multicast
///     - `b0`
/// - Local (U/L) bit= univeral/local
///     - `b1`
#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = &self.0;
        // Convert each byte to hexadecimal
        write!(f, "{}", hexfmt(octet))
    }
}

impl MacAddress {
    /// Create a random local, multicast `MacAddress`.
    pub fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        // Set local/unicast
        octets[0] |= LOCAL;
        octets[0] &= !UNICAST;
        MacAddress { 0: octets }
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress { 0: self.0 }
    }
}

/// Format a sequence of bytes as a `:` separated hexadecimal string.
fn hexfmt(x: &[u8]) -> String {
    x.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":")
}

/// Yes, I'm aware this isn't exactly extensive testing...
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_mac_address_is_local_and_multicast() {
        //
        // fn is_unicast(&self) -> bool {
        //     (self.0[0] & UNICAST) == UNICAST
        // }
        let addr = MacAddress::new();
        assert_eq!(addr.0[0] & LOCAL, LOCAL);
        assert_ne!(addr.0[0] & UNICAST, UNICAST);
    }

    #[test]
    fn test_hexfmt() {
        let s = hexfmt(&[220, 141, 146, 221, 205, 217]);
        assert_eq!(s, "DC:8D:92:DD:CD:D9");
    }
}
