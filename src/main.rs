extern crate regex;
extern crate socks;

use std::io::{Error, Read, Write};
use std::net::SocketAddr;
use std::str::FromStr;

use regex::Regex;

use socks::{Socks5Stream, TargetAddr};

fn main() {
    //connect to site through proxy
    let proxy_addr = SocketAddr::from_str("127.0.0.1:9050").unwrap();

    let (mut onion_addrs, mut crawled_addrs) = (vec!("xmh57jrzrnw6insl".to_string()), Vec::new());
    while onion_addrs.len() != 0 {
        let onion_addr = onion_addrs.pop().unwrap();
        let target_addr = TargetAddr::Domain(format!("{}.onion", onion_addr).to_string(), 80);

        match crawl(&proxy_addr, target_addr) {
            Ok(found_addrs) => {
                for addr in found_addrs {
                    println!("found: {}", addr);
                    onion_addrs.push(addr);
                }
            },
            Err(_) => println!("error crawling {}", onion_addr),
        }

        crawled_addrs.push(onion_addr);
    }

    println!("crawled {} addresses", crawled_addrs.len());
}

fn crawl(proxy_addr: &SocketAddr, target_addr: TargetAddr) -> Result<Vec<String>, Error> {
    //open stream
    let mut stream = try!(Socks5Stream::connect(proxy_addr, target_addr));

    //write HTTP GET
    try!(stream.write_all(b"GET / HTTP/1.0\r\n\r\n"));

    //read response
    let mut buf = Vec::new();
    let _ = try!(stream.read_to_end(&mut buf));
    let response_str = try!(std::str::from_utf8(&buf));

    //search for .onion addresses in response
    let re = Regex::new("(http|https)://(.{16}).onion").unwrap();
    let mut found_addrs = Vec::new();
    for onion_addr in re.captures_iter(response_str) {
        found_addrs.push(onion_addr.at(2).unwrap().to_string());
    }

    Ok(found_addrs)
}
