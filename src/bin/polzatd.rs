extern crate capnp;
extern crate docopt;
extern crate grpc;
extern crate polzat;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use capnp::message::ReaderOptions;
use capnp::serialize::{read_message, write_message};
use docopt::Docopt;
use polzat::{create_stats_reply};
use polzat::fetcher::{Fetcher, WgetFetcher};
use polzat::frontier::{FIFOFrontier, Frontier};
use polzat::fetcher::{FetcherV2, LibcurlFetcher};
use polzat::frontier::{FrontierV2, PriorityFrontier};
use polzat::link_extractor::IterativeExtractor;
use polzat::message_capnp::polzat_message::message_type::{CrawlRequest, StatsRequest};

use grpc::error::GrpcError;
use grpc::result::GrpcResult;
use polzat::polzat_pb::{ScheduleTaskReply, ScheduleTaskRequest};
use polzat::polzat_pb_grpc::{Polzat, PolzatServer};

const USAGE: &'static str = "
PolzatD application used for distributed web crawling and scraping

Usage:
    polzatd [--thread-count=<t>] [--port=<p>]
    polzatd (-h | --help)

Options:
    --thread-count=<t>          Number for fetching threads [default: 10].
    --port=<p>                  Port of capnproto socket [default: 12289].
    -h --help                   Display this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_thread_count: usize,
    flag_port: u16,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let fetcher = LibcurlFetcher::new();
    let frontier = PriorityFrontier::new();

    let _polzatd = PolzatServer::new(args.flag_port, PolzatD::new());
    loop {
        std::thread::park();
    }
}

struct PolzatD {

}

impl PolzatD {
    pub fn new() -> PolzatD {
        PolzatD {
        }
    }
}

impl Polzat for PolzatD {
    fn ScheduleTask(&self, request: ScheduleTaskRequest) -> GrpcResult<ScheduleTaskReply> {
        println!("scheduleing task: {:?}", request);

        Err(GrpcError::Other("unimplemented!"))
    }
}

/*const USAGE: &'static str = "
PolzatD application used for distributed web crawling and scraping

Usage:
    polzatd [--site-directory=<dir>] [--thread-count=<thread>] [--ip-address=<ip>] [--port=<port>]
    polzatd (-h | --help)

Options:
    --site-directory=<dir>      Directory to download sites to [default: /tmp/sites].
    --thread-count=<thread>     Number of fetching threads [default: 10].
    --ip-address=<ip>           IP address of application [default: 127.0.0.1].
    --port=<port>               Port of application [default: 12289].
    -h --help                   Show this screen.
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

                //read polzat message
                let reader = match read_message(&mut stream, ReaderOptions::default()) {
                    Ok(reader) => reader,
                    Err(_) => panic!("unable to read message from tcp stream"),
                };

                let msg = match reader.get_root::<polzat::message_capnp::polzat_message::Reader>() {
                    Ok(msg) => msg,
                    Err(_) => panic!("unable to parse polzat message"),
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
}*/
