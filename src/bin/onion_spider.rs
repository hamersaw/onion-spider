extern crate capnp;
extern crate docopt;
extern crate onion_spider;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use capnp::message::ReaderOptions;
use capnp::serialize::{read_message, write_message};
use docopt::Docopt;
use onion_spider::{create_stats_reply};
use onion_spider::fetcher::{Fetcher, WgetFetcher};
use onion_spider::frontier::{FIFOFrontier, Frontier};
use onion_spider::link_extractor::IterativeExtractor;
use onion_spider::message_capnp::onion_spider_message::message_type::{CrawlRequest, StatsRequest};

const USAGE: &'static str = "
OnionSpider application used for distributed crawling of TOR hidden services

Usage:
    onion_spider [--site-directory=<dir>] [--thread-count=<thread>] [--ip-address=<ip>] [--port=<port>]
    onion_spider (-h | --help)

Options:
    -h --help                   Show this screen.
    --site-directory=<dir>      Directory to download sites to [default: /tmp/sites].
    --thread-count=<thread>     Number of fetching threads [default: 10].
    --ip-address=<ip>           IP address of application [default: 127.0.0.1].
    --port=<port>               Port of application [default: 12289].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_site_directory: String,
    flag_thread_count: i32,
    flag_ip_address: String,
    flag_port: i32,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    //create crawling structures
    let frontier = Arc::new(FIFOFrontier::new());

    for i in 0..args.flag_thread_count {
        let thread_frontier = frontier.clone();
        let thread_site_directory = args.flag_site_directory.clone();

        thread::spawn(move || {
            let fetcher = WgetFetcher::new(thread_site_directory.clone(), thread_frontier, Box::new(IterativeExtractor::new(thread_site_directory)));
            match fetcher.start() {
                Ok(_) => {},
                Err(e) => panic!("unable to run fetcher {}: {}", i, e),
            }
        });
    }

    //open tcp listener
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
            let thread_frontier = frontier.clone();

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
                        //handle a crawl request
                        let crawl_request = crawl_request_result.unwrap();

                        for i in 0..crawl_request.len() {
                            let _ = thread_frontier.add_site(crawl_request.get(i).unwrap());
                        }
                    },
                    Ok(StatsRequest(_)) => {
                        //handle a stats request
                        let stats_reply = match create_stats_reply(thread_frontier.len()) {
                            Ok(stats_reply) => stats_reply,
                            Err(e) => panic!("unable to create stats reply: {}", e),
                        };

                        match write_message(&mut stream, &stats_reply) {
                            Err(e) => panic!("unable to write stats reply: {}", e),
                            _ => {},
                        };
                    },
                    Ok(_) => panic!("unexpected message type"),
                    Err(e) => panic!("unknown error: {}", e),
                }
            });
        }
    });

    handle.join().unwrap();
}

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
