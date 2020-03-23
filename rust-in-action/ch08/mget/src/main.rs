#[macro_use]
extern crate log;
extern crate env_logger;

use smoltcp::phy::TapInterface;
use structopt::StructOpt;
use url::Url;

mod dns;
mod ethernet;
mod http;

#[derive(Debug, StructOpt)]
#[structopt(name = "mget", about = "GET a webpage, manually")]
struct Opt {
    #[structopt(parse(try_from_str = parse_url))]
    url: Url,
    #[structopt(parse(try_from_str = parse_tap_device))]
    tap_device: TapInterface,
    #[structopt(short = "s", default_value = "1.1.1.1")]
    dns_server: std::net::Ipv4Addr,
}

fn parse_tap_device(name: &str) -> Result<TapInterface, String> {
    match TapInterface::new(name) {
        Ok(tap) => Ok(tap),
        Err(e) => Err(format!(
            "{} - Does TAP device '{}' exist?",
            e.to_string(),
            name
        )),
    }
}

fn parse_url(url: &str) -> Result<Url, String> {
    let url = Url::parse(url).map_err(|e| e.to_string())?;
    if url.scheme() != "http" {
        Err(String::from("Only HTTP protocol supported"))
    } else if let None = url.host_str() {
        Err(String::from("Domain name required"))
    } else {
        Ok(url)
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    // Existence checked during CLI opt parsing
    let domain_name = opt.url.host_str().unwrap();
    debug!("Domain name: {}", domain_name);

    debug!("TAP device: {:?}", opt.tap_device);

    let addr = dns::resolve(opt.dns_server.into(), domain_name)
        .unwrap()
        .unwrap();
    debug!("DNS resolution: {} -> {}", domain_name, addr);

    let mac = ethernet::MacAddress::new().into();
    debug!("mac: {}", mac);

    http::get(opt.tap_device, mac, addr, opt.url).unwrap();
}
