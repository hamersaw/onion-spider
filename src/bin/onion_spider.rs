extern crate capnp;
extern crate docopt;
extern crate onion_spider;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::thread;

use capnp::NotInSchema;
use capnp::message::ReaderOptions;
use capnp::serialize::{read_message, write_message};
use docopt::Docopt;
use onion_spider::message_capnp::onion_spider_message::message_type::{CrawlRequest, StatsRequest};

const USAGE: &'static str = "
OnionSpider application used for distributed crawling of TOR hidden services

Usage:
    onion_spider [--ip-address=<ip>] [--port=<port>]
    onion_spider (-h | --help)

Options:
    -h --help           Show this screen.
    --ip-address=<ip>   IP address of application [default: 127.0.0.1].
    --port=<port>       Port of application [default: 12289].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_ip_address: String,
    flag_port: i32,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let app_addr = match SocketAddr::from_str(&format!("{}:{}", args.flag_ip_address, args.flag_port)) {
        Ok(app_addr) => app_addr,
        Err(_) => panic!("unable to parse ip address {};{}", args.flag_ip_address, args.flag_port),
    };

    let listener = match TcpListener::bind(app_addr) {
        Ok(listener) => listener,
        Err(_) => panic!("unable to bind to tcp socket {}:{}", args.flag_ip_address, args.flag_port),
    };

    //start tcp listener thread
    let handle = thread::spawn(move || {
        for stream in listener.incoming() {
            thread::spawn(move || {
                let mut stream = stream.unwrap();

                //read spider onion message
                let reader = match read_message(&mut stream, ReaderOptions::default()) {
                    Ok(reader) => reader,
                    Err(_) => panic!("unable to read message from tcp stream"),
                };

                let msg = match reader.get_root::<onion_spider::message_capnp::onion_spider_message::Reader>() {
                    Ok(msg) => msg,
                    Err(_) => panic!("unable to parse onion spider message"),
                };

                match msg.get_message_type().which() {
                    Ok(CrawlRequest(crawl_request_result)) => {
                        

                        /*let mut iter = crawl_request_result.unwrap().iter();
                        loop {
                            match iter.next() {
                                Some(site) => {

                                },
                                None => break,
                            }
                        }*/
                    },
                    Ok(_) => panic!("unknown onion spider message type"),
                    Err(e) => panic!("unknown error: {}", e),
                }
            });
        }
    });

    handle.join().unwrap();
}

/*use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::sync::mpsc;
use std::thread;

use downloader::wget_download;

static THREAD_COUNT: i32 = 10;

fn main() {
    let mut open_chans = Vec::new();
    let mut site_chans = Vec::new();
    let (done_tx, done_rx) = mpsc::channel();

    //start up all of the threads
    for id in 0..THREAD_COUNT {
        open_chans.push(id);

        let (site_tx, site_rx) = mpsc::channel();
        site_chans.push(site_tx);
        let thread_done_tx = done_tx.clone();

        thread::spawn(move || {
            println!("started thread {}", id);

            loop {
                match site_rx.recv() {
                    Ok(site) => {
                        wget_download(site).unwrap();
                        thread_done_tx.send(id).unwrap();
                    }
                    Err(_) => {
                        println!("error recv thread {}", id);
                        break
                    },
                }
            }
        });
    }

    //read in onion addresses
    let mut site_buffer: Vec<String> = Vec::new();
    let file = File::open("examples/sites.txt").unwrap();
    let reader = BufReader::new(file);

    for site in reader.lines() {
        site_buffer.push(site.unwrap());
    }

    //push initial onion addresses to work channel
    let mut active_sites = 0;
    while open_chans.len() > 0 {
        if site_buffer.len() == 0 {
            break
        }

        let chan_index = open_chans.pop().unwrap();
        let site = site_buffer.pop().unwrap();

        site_chans[chan_index as usize].send(site).unwrap();
        active_sites += 1;
    }

    //
    while active_sites > 0 {
        let chan_id = done_rx.recv().unwrap();
        
        println!("completed:{}", chan_id);
        active_sites -= 1;
    }
}*/

/*fn main() {
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
}*/
