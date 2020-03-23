//! # DNS utility to resolve the IP address for domain names
//!
//! Uses DNS over UDP ([RFC 1035](https://tools.ietf.org/html/rfc1035#section-4.2.1))
//! to resolve a domain name to an IP address.
//!
use std::{
    error, fmt,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use rand;
use trust_dns::{
    op::{Message, MessageType, OpCode, Query},
    proto::error::ProtoError,
    rr::{domain::Name, record_type::RecordType},
    serialize::binary::*,
};

/// Maximum message size for DNS over UDP
/// https://tools.ietf.org/html/rfc1035#section-4.2.1
const UDP_MAX_MESSAGE_SIZE: usize = 512;

/// Generate a random non-zero integer message ID
fn message_id() -> u16 {
    let candidate = rand::random();
    match candidate {
        0 => message_id(),
        _ => candidate,
    }
}

// TODO: impl Into for each variant to remove need for map_err in resolve
#[derive(Debug)]
pub enum DnsError {
    ParseDomainName(ProtoError),
    ParseDnsServerAddress(std::net::AddrParseError),
    Encoding(ProtoError),
    Decoding(ProtoError),
    Network(std::io::Error),
    Sending(std::io::Error),
    Receiving(std::io::Error),
}

impl fmt::Display for DnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

// Use default implementations
impl error::Error for DnsError {}

pub fn resolve(
    dns_server_address: std::net::Ipv4Addr,
    domain_name: &str,
) -> Result<Option<std::net::IpAddr>, DnsError> {
    // Convert string inputs into specific data types for representing domain
    // names and addresses
    let domain_name = Name::from_ascii(&domain_name).map_err(DnsError::ParseDomainName)?;
    let dns_server: SocketAddr = format!("{}:53", dns_server_address)
        .parse()
        .map_err(DnsError::ParseDnsServerAddress)?;

    // TODO: Why 50
    let mut request_buffer: Vec<u8> = Vec::with_capacity(50);
    // Fill response buffer as `recv_from` relies on the *length* of the buffer
    // to determine how many bytes to read from the network
    // `with_capacity` would result in the buffer having length 0
    let mut response_buffer: [u8; UDP_MAX_MESSAGE_SIZE] = [0; UDP_MAX_MESSAGE_SIZE];

    let mut request = Message::new();
    request
        .set_id(message_id())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        // Query for IPv4 addresses
        .add_query(Query::query(domain_name, RecordType::A))
        // If the initial DNS server cannot resolve the domain name, allow it
        // to query other DNS servers on our behalf
        .set_recursion_desired(true);

    // Set up local UDP server to communicate with remote DNS server
    // UDP **does not** support *duplex* (long-lived) communication -> both
    // parties must act as clients *and* servers
    let localhost = UdpSocket::bind("0.0.0.0:0").map_err(DnsError::Network)?;
    localhost
        .set_read_timeout(Some(Duration::from_secs(5))) // TODO configurable
        .map_err(DnsError::Network)?;
    localhost
        .set_nonblocking(false)
        .map_err(DnsError::Network)?;

    // Convert message to raw bytes ready to be sent via UDP
    let mut encoder = BinEncoder::new(&mut request_buffer);
    request.emit(&mut encoder).map_err(DnsError::Encoding)?;

    let _n_sent_bytes = localhost
        .send_to(&request_buffer, dns_server)
        .map_err(DnsError::Sending)?;

    // Ignore packets from unexpected IP addresses. There is a *small* chance
    // that a UDP message will be received on our port from some unknown sender
    loop {
        let (_n_recv_bytes, remote_port) = localhost
            .recv_from(&mut response_buffer)
            .map_err(DnsError::Receiving)?;
        if remote_port == dns_server {
            break;
        }
    }

    let response = Message::from_vec(&response_buffer).map_err(DnsError::Decoding)?;

    for answer in response.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            // TODO: Error instead of expect?
            let server_ip = resource.to_ip_addr().expect("invalid IP address received");
            return Ok(Some(server_ip));
        }
    }

    Ok(None)
}
