//! # `macgen` - a MAC address generator
//!
//! Generates random locally assigned, unicast MAC addresses.
//!
//! ## MAC Address
//!
//! Media access control address is a *unique* identifier assigned to a NIC
//! to be used as a network address.
//! 2 forms:
//! - Universally administered (universal)
//!   - Set when devices are manufactured
//!   - Consist of:
//!     - Prefix assigned by IEEE Registration Authority to the manufacturer
//!     - Manufacturer-chosen scheme for the remaining bits
//! - Locally administered (local)
//!   - Created by devices to set their own MAC address *without* registration
//!
//! 2 modes:
//! - Unicast
//!   - Transport information between two points *in direct contact*
//!     - e.g. laptop <-> router
//!   - Frames are accepted *only* by the NIC with the matching MAC address
//! - Multicast
//!   - NICs choose whether to accept a frame based on criteria *other* than
//!     matching MAC address
//!
//! MAC addresses are 48 bits wide. The structure depends on whether the
//! address is unversal or local:
//!
//! ```text
//! +--------+--------+--------+--------+--------+--------+
//! |||||||||||||||||||||||||||||||||||||||||||||||||||||||
//! +--------+--------+--------+--------+--------+--------+
//!
//! |______..__________________|__________________________| Universal
//!          Organisation                Device
//!
//! |______..__________________|__________________________| Local
//!                          Device
//!
//! |......__..................|..........................| Common fields
//!       Flags
//!
//! |        |
//! |        +---------------
//! |                       |
//! v                       v
//! +-----------------------+
//! |b7|b6|b5|b4|b3|b2|b1|b0|
//! +-----------------------+
//! ```
//!
//! The flags indicate unicast/multicast and universal/local:
//! - Group bit = unicast/multicast
//!     - `b0`
//! - Local (U/L) bit= univeral/local
//!     - `b1`
//!
//!
//! ## TODO
//!
//! - Provide options to toggle group bit / local bit when generating
//! - Generate universal addresses for a specified organisation using IEEE
//!   assignments e.g. https://regauth.standards.ieee.org/standards-ra-web/pub/view.html#registries

use rand::RngCore;
use std::fmt;
use std::fmt::Display;

const LOCAL: u8 = 0b_0000_0010;
const UNICAST: u8 = 0b_0000_0001;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = &self.0;
        // Convert each byte to hexadecimal
        write!(f, "{}", hexfmt(octet))
    }
}

impl MacAddress {
    /// Create a random local, unicast `MacAddress`.
    fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        // Set local/unicast
        octets[0] |= LOCAL | UNICAST;
        MacAddress { 0: octets }
    }

    fn is_local(&self) -> bool {
        (self.0[0] & LOCAL) == LOCAL
    }

    fn is_unicast(&self) -> bool {
        (self.0[0] & UNICAST) == UNICAST
    }
}

/// Format a sequence of bytes as a `:` separated hexadecimal string.
fn hexfmt(x: &[u8]) -> String {
    x.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":")
}

fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac: {}", mac);
}

/// Yes, I'm aware this isn't exactly extensive testing...
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_mac_address_is_local_and_unicast() {
        let addr = MacAddress::new();
        assert!(addr.is_local());
        assert!(addr.is_unicast());
    }

    #[test]
    fn test_hexfmt() {
        let s = hexfmt(&[220, 141, 146, 221, 205, 217]);
        assert_eq!(s, "DC:8D:92:DD:CD:D9");
    }
}
