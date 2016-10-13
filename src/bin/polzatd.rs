extern crate docopt;
extern crate grpc;
extern crate polzat;
extern crate rustc_serialize;

use std::sync::{Arc, RwLock};

use docopt::Docopt;
use polzat::fetcher::{Fetcher, LibcurlFetcher};
use polzat::frontier::{Frontier, PriorityFrontier};
use polzat::link_extractor::IterativeExtractor;

use grpc::error::GrpcError;
use grpc::result::GrpcResult;
use polzat::{PolzatTask, Operation, UrlType};
use polzat::polzat_pb::{ScheduleTaskReply, ScheduleTaskRequest, ScheduleTaskRequest_UrlType, ScheduleTaskRequest_Operation};
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
    let frontier = Arc::new(RwLock::new(PriorityFrontier::new()));

    let _polzatd = PolzatServer::new(args.flag_port, PolzatD::new(frontier.clone()));
    loop {
        std::thread::park();
    }
}

struct PolzatD {
    frontier: Arc<RwLock<PriorityFrontier>>,
}

impl PolzatD {
    pub fn new(frontier: Arc<RwLock<PriorityFrontier>>) -> PolzatD {
        PolzatD {
            frontier: frontier,
        }
    }
}

impl Polzat for PolzatD {
    fn ScheduleTask(&self, request: ScheduleTaskRequest) -> GrpcResult<ScheduleTaskReply> {
        let polzat_task = PolzatTask::new(
                request.get_execution_id(), 
                request.get_priority() as u8,
                request.get_url().to_owned(),
                match request.get_url_type() {
                    ScheduleTaskRequest_UrlType::Web => UrlType::Web,
                    ScheduleTaskRequest_UrlType::TorHiddenService => UrlType::TorHiddenService,
                },
                match request.get_operation() {
                    ScheduleTaskRequest_Operation::Crawl => Operation::Crawl,
                    ScheduleTaskRequest_Operation::Scrape => Operation::Scrape,
                },
            );
        
        let mut frontier = self.frontier.write().unwrap();
        match frontier.push(polzat_task) {
            Ok(_) => Ok(polzat::create_schedule_task_reply()),
            Err(_) => Err(GrpcError::Other("unable to push task to frontier")),
        }
    }
}
