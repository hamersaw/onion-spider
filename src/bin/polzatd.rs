extern crate docopt;
extern crate grpc;
extern crate polzat;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use docopt::Docopt;
use polzat::fetcher::{Fetcher, WgetFetcher};
use polzat::frontier::{FIFOFrontier, Frontier};
use polzat::fetcher::{FetcherV2, LibcurlFetcher};
use polzat::frontier::{FrontierV2, PriorityFrontier};
use polzat::link_extractor::IterativeExtractor;

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
