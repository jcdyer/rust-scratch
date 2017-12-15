#![feature(ip_constructors)]

use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    println!("IPv4 localhost: {}", Ipv4Addr::localhost());
    println!("IPv4 unspecified: {}", Ipv4Addr::unspecified());
    println!("IPv6 localhost: {}", Ipv6Addr::localhost());
    println!("IPv6 unspecified: {}", Ipv6Addr::unspecified());
}
