//! # Simple DNS CLI utility that can resolve the IP address for domain names
//!
//! `resolve` uses DNS over UDP ([RFC 1035](https://tools.ietf.org/html/rfc1035#section-4.2.1))
//! to resolve a domain name to an IP address.
//!
//! ## Context within chapter
//!
//! TCP requires IP addresses to specify where to connect to - it does not
//! 'understand' hostnames. Domain Name resolution (DNS) enables hostnames
//! (www.rustinaction.com) to be *resolved* to IP addresses. This is used to
//! enable a TCP connection to be established from hostnames instead of
//! requiring clients to specify (and therefore know) IP addresses.
//!
//!
use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use clap::{App, Arg};
use rand;
use trust_dns::{
    op::{Message, MessageType, OpCode, Query},
    rr::{domain::Name, record_type::RecordType},
    serialize::binary::*,
};

/// Maximum message size for DNS over UDP
/// https://tools.ietf.org/html/rfc1035#section-4.2.1
const UDP_MAX_MESSAGE_SIZE: usize = 512;

fn main() {
    let app = App::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(
            Arg::with_name("dns-server")
                .short("s")
                .default_value("1.1.1.1"), // Cloudflare public DNS
        )
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    let domain_name_raw = app.value_of("domain-name").unwrap();
    // Convert string into a specific data type for representing domain names
    let domain_name = Name::from_ascii(&domain_name_raw).unwrap();

    let dns_server_raw = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse()
        .expect("invalid address");

    // Not strictly necessary to reserve the max message size, this could be
    // more intelligent and construct the correct size buffer as needed.
    // Keeping it simple here.
    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(UDP_MAX_MESSAGE_SIZE);
    // Fill response buffer as `recv_from` relies on the *length* of the buffer
    // to determine how many bytes to read from the network
    // `with_capacity` would result in the buffer having length 0
    let mut response_as_bytes: [u8; UDP_MAX_MESSAGE_SIZE] = [0; UDP_MAX_MESSAGE_SIZE];

    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        // Query for IPv4 addresses
        .add_query(Query::query(domain_name, RecordType::A))
        // If the initial DNS server cannot resolve the domain name, allow it
        // to query other DNS servers on our behalf
        .set_recursion_desired(true);

    // Convert message to raw bytes ready to be sent via UDP
    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    // Set up local UDP server to communicate with remote DNS server
    // UDP **does not** support *duplex* (long-lived) communication -> both
    // parties must act as clients *and* servers
    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _n_sent_bytes = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    let (_n_recv_bytes, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip = resource.to_ip_addr().expect("invalid IP address received");
            println!("{}", ip.to_string());
        }
    }
}
