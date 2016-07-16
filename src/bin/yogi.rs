extern crate capnp;
extern crate docopt;
extern crate onion_spider;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;

use capnp::message::ReaderOptions;
use capnp::serialize::{read_message, write_message};
use docopt::Docopt;
use onion_spider::{create_crawl_request, create_stats_request};
use onion_spider::message_capnp::onion_spider_message::message_type::{StatsReply};

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

        let crawl_request = match create_crawl_request(sites) {
            Ok(crawl_request) => crawl_request,
            Err(_) => panic!("unable to create crawl request message"),
        };

        match write_message(&mut stream, &crawl_request) {
            Err(_) => panic!("unable to send crawl request message"),
            _ => {},
        }

        //TODO recv
    } else if args.cmd_stats {
        //issue stats request
        let stats_request = match create_stats_request() {
            Ok(stats_request) => stats_request,
            Err(_) => panic!("unable to create stats request message"),
        };

        match write_message(&mut stream, &stats_request) {
            Err(_) => panic!("unable to send stats request message"),
            _ => {},
        }

        //read spider onion message
        let reader = match read_message(&mut stream, ReaderOptions::default()) {
            Ok(reader) => reader,
            Err(_) => panic!("unable to read message from tcp stream"),
        };

        let msg = match reader.get_root::<onion_spider::message_capnp::onion_spider_message::Reader>() {
            Ok(msg) => msg,
            Err(_) => panic!("unable to parse onion spider message"),
        };

        //
        match msg.get_message_type().which() {
            Ok(StatsReply(stats_reply)) => {
                println!("frontier size: {}", stats_reply.get_frontier_size());
            },
            Err(e) => panic!("unable to get message type stats request reply: {}", e),
            _ => panic!("expecting a stats reply from a stats request"),
        }
    }
}
