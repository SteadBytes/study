//! HTTP request handling
use crate::ethernet;
use smoltcp::time::Instant;
use smoltcp::{
    iface::{EthernetInterfaceBuilder, NeighborCache, Routes},
    phy::{wait as phy_wait, TapInterface},
    socket::{SocketSet, TcpSocket, TcpSocketBuffer},
    wire::{EthernetAddress, IpCidr, Ipv4Address},
};
use std::collections::BTreeMap;
use std::fmt;
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(std::str::Utf8Error),
    Ethernet(ethernet::Error),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(error: smoltcp::Error) -> Self {
        UpstreamError::Network(error)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(error: std::str::Utf8Error) -> Self {
        UpstreamError::Content(error)
    }
}

impl From<ethernet::Error> for UpstreamError {
    fn from(error: ethernet::Error) -> Self {
        UpstreamError::Ethernet(error)
    }
}

impl std::error::Error for UpstreamError {}

type Ipv4Bytes = [u8; 4];

/// CIDR assigned to host
pub const HOST_CIDR: (Ipv4Bytes, u8) = ([192, 168, 69, 1], 24);

/// IP of TAP device
pub const TAP_IP: Ipv4Bytes = [192, 168, 69, 100];

/// Perform an HTTP GET for `url` and print the response. Note: The IP assigned
/// to `tap` **must** match [`TAP_IP`].
pub fn get(
    tap: TapInterface,
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    let domain_name = url.host_str().ok_or(UpstreamError::InvalidUrl)?;

    // Set up TCP socket
    // TODO: Why this size?
    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

    let mut sockets = SocketSet::new(vec![]);
    let tcp_handle = sockets.add(tcp_socket);

    let fd = tap.as_raw_fd(); // Grab fd now before `tap` is moved to `iface`

    // Set up Ethernet interface

    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    // Assign IP to the host
    let (host_ip, slash) = HOST_CIDR;
    let ip_addrs = [IpCidr::new(Ipv4Address::from_bytes(&host_ip).into(), slash)];
    let mut routes = Routes::new(BTreeMap::new());
    // IP of the TAP device
    let default_v4_gateway = Ipv4Address::from_bytes(&TAP_IP);
    routes
        .add_default_ipv4_route(default_v4_gateway)
        .map_err(UpstreamError::Network)?;
    let mut iface = EthernetInterfaceBuilder::new(tap)
        .ethernet_addr(mac)
        .neighbor_cache(neighbor_cache)
        .ip_addrs(ip_addrs)
        .routes(routes)
        .finalize();

    let http_header = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path(),
        domain_name
    );

    // State machine loop
    let mut state = HttpState::Connect;
    loop {
        let timestamp = Instant::now();
        match iface.poll(&mut sockets, timestamp) {
            Ok(_) => {}
            Err(e) => {
                error!("poll error: {:?}", e);
            }
        }

        {
            let mut socket = sockets.get::<TcpSocket>(tcp_handle);

            state = match state {
                HttpState::Connect if !socket.is_active() => {
                    info!("connecting");
                    socket.connect((addr, url.port().unwrap_or(80)), ethernet::free_tcp_port()?)?;
                    HttpState::Request
                }
                HttpState::Request if socket.may_send() => {
                    info!("sending request");
                    socket.send_slice(http_header.as_ref())?;
                    HttpState::Response
                }
                HttpState::Response if socket.can_recv() => {
                    socket.recv(|raw_data| {
                        let output = String::from_utf8_lossy(raw_data);
                        println!("{}", output);
                        (raw_data.len(), ())
                    })?;
                    HttpState::Response
                }
                HttpState::Response if !socket.may_recv() => {
                    info!("received complete response");
                    break;
                }
                _ => state,
            }
        }

        let d = iface.poll_delay(&sockets, timestamp);
        debug!("Delay: {:?}", d);
        phy_wait(fd, d).expect("wait error");
    }

    Ok(())
}
