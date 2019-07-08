use pnet::datalink::{self, NetworkInterface};
use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpType};
use pnet::packet::Packet;

use std::env;
use std::io;
use std::net::{IpAddr, ToSocketAddrs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let interfaces = datalink::interfaces();

    let ip = match args.len() {
        2 => resolve(&args[1].as_str()).unwrap(),
        _ => panic!("Please specify host or ip"),
    };

    println!("{:?}", &ip[0]);
    println!("{:?}", interfaces);
}

fn resolve(host: &str) -> io::Result<Vec<IpAddr>> {
    (host, 0)
        .to_socket_addrs()
        .map(|iter| iter.map(|socket_address| socket_address.ip()).collect())
}
