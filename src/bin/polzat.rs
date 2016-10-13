extern crate capnp;
extern crate docopt;
extern crate polzat;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;

use capnp::message::ReaderOptions;
use capnp::serialize::{read_message, write_message};
use docopt::Docopt;
use polzat::{UrlType, Operation};
use polzat::polzat_pb_grpc::{Polzat, PolzatClient};
use polzat::{create_crawl_request, create_stats_request};
use polzat::message_capnp::polzat_message::message_type::{StatsReply};

const USAGE: &'static str = "
Interact with PolzatD application

Usage:
    polzat crawl <url> [--ip-address=<ip>] [--port=<port>]
    polzat scrape <url> [--ip-address=<ip>] [--port=<port>]
    polzat (-h | --help)

Options:
    -h --help           Show this screen.
    --ip-address=<ip>   IP address of application [default: 127.0.0.1].
    --port=<port>       Port of application [default: 12289].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_crawl: bool,
    arg_url: String,
    flag_ip_address: String,
    flag_port: u16,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let client = PolzatClient::new(&args.flag_ip_address, args.flag_port, false).unwrap();

    if args.cmd_crawl {
        let request = polzat::create_schedule_task_request(0, 0, &args.arg_url, UrlType::Web, Operation::Crawl);
        let response = client.ScheduleTask(request);

        println!("response: {:?}", response);
    } else if args.cmd_scrape {
        let request = polzat::create_schedule_task_request(0, 0, &args.arg_url, UrlType::Web, Operation::Scrape);
        let response = client.ScheduleTask(request);

        println!("response: {:?}", response);
    }
}

/*fn main() {
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

        //read polzat message
        let reader = match read_message(&mut stream, ReaderOptions::default()) {
            Ok(reader) => reader,
            Err(_) => panic!("unable to read message from tcp stream"),
        };

        let msg = match reader.get_root::<polzat::message_capnp::polzat_message::Reader>() {
            Ok(msg) => msg,
            Err(_) => panic!("unable to parse polzat message"),
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
}*/
