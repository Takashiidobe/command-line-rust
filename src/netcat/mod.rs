use anyhow::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use anyhow::Result;
use clap::{App, Arg};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Config {
    pub listen: bool,
    pub protocol: Protocol,
    pub ip_addr: SocketAddr,
}

pub fn get_args() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Netcat")
        .arg(
            Arg::with_name("listen")
                .short('l')
                .takes_value(false)
                .required(false)
                .help("Listen instead of creating a client"),
        )
        .arg(
            Arg::with_name("protocol")
                .short('u')
                .takes_value(false)
                .required(false)
                .help("set protocol to udp"),
        )
        .arg(
            Arg::with_name("ipv4")
                .takes_value(false)
                .short('4')
                .required(false)
                .help("sets ip version to 4"),
        )
        .arg(
            Arg::with_name("ipv6")
                .takes_value(false)
                .short('6')
                .help("sets ip version to 6")
                .required(false)
                .conflicts_with("ipv4"),
        )
        .arg(Arg::with_name("destination").takes_value(true))
        .arg(Arg::with_name("port").takes_value(true))
        .get_matches();

    let destination = matches
        .value_of("destination")
        .expect("Expected a hostname to be provided");
    let port = matches
        .value_of("port")
        .expect("Expected a port to be provided");
    let listen = matches.is_present("listen");
    let protocol = if matches.is_present("protocol") {
        Protocol::Udp
    } else {
        Protocol::Tcp
    };
    let destination = if destination == "localhost" {
        Ipv4Addr::LOCALHOST
    } else {
        destination.parse().expect("Could not parse destination")
    };
    let port = port.parse::<u16>().expect("Invalid port number");
    Config {
        listen,
        protocol,
        ip_addr: format!("{}:{}", destination, port)
            .parse::<SocketAddr>()
            .expect("Failed to parse SocketAddr"),
    }
}
