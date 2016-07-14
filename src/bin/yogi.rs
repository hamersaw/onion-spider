extern crate capnp;
extern crate docopt;
extern crate onion_spider;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;

use capnp::serialize::{read_message, write_message};
use docopt::Docopt;
use onion_spider::{create_crawl_request_msg, create_stats_request_msg};

const USAGE: &'static str = "
Interact with SpiderOnion application

Usage:
    yogi crawl <site> [--ip-address=<ip>] [--port=<port>]
    yogi stats
    yogi (-h | --help)

Options:
    -h --help           Show this screen.
    --ip-address=<ip>   IP address of application [default: 127.0.0.1].
    --port=<port>       Port of application [default: 12289].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_crawl: bool,
    cmd_stats: bool,
    arg_site: Option<Vec<String>>,
    flag_ip_address: String,
    flag_port: i32,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    //open tcp stream
    let app_addr = match SocketAddr::from_str(&format!("{}:{}", args.flag_ip_address, args.flag_port)) {
        Ok(app_addr) => app_addr,
        Err(_) => panic!("unable to parse ip address {};{}", args.flag_ip_address, args.flag_port),
    };

    let mut stream = match TcpStream::connect(app_addr) {
        Ok(stream) => stream,
        Err(_) => panic!("unable to connect to tcp stream {}:{}", args.flag_ip_address, args.flag_port),
    };

    if args.cmd_crawl {
        //issue crawl request
        let sites = match args.arg_site {
            Some(sites) => sites,
            None => panic!("unable to issue empty crawl request, 0 sites were provided"),
        };

        let crawl_request_msg = match create_crawl_request_msg(sites) {
            Ok(crawl_request_msg) => crawl_request_msg,
            Err(_) => panic!("unable to create crawl request message"),
        };

        match write_message(&mut stream, &crawl_request_msg) {
            Err(_) => panic!("unable to send crawl request message"),
            _ => {},
        }

        //TODO recv
    } else if args.cmd_stats {
        //issue stats request
        let stats_request_msg = match create_stats_request_msg() {
            Ok(stats_request_msg) => stats_request_msg,
            Err(_) => panic!("unable to create stats request message"),
        };

        match write_message(&mut stream, &stats_request_msg) {
            Err(_) => panic!("unable to send stats request message"),
            _ => {},
        }

        //TODO recv
    }
}
