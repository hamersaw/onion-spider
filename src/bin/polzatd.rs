extern crate docopt;
extern crate grpc;
extern crate polzat;
extern crate rustc_serialize;
extern crate threadpool;

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use docopt::Docopt;
use grpc::error::GrpcError;
use grpc::result::GrpcResult;
use polzat::{FetcherType, LinkExtractorType, PolzatTask, Operation, UrlType};
use polzat::frontier::{Frontier, PriorityFrontier};
use polzat::polzat_pb::{ScheduleTaskReply, ScheduleTaskRequest, ScheduleTaskRequest_UrlType, ScheduleTaskRequest_Operation};
use polzat::polzat_pb_grpc::{Polzat, PolzatServer};
use threadpool::ThreadPool;

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

    //let fetcher = LibcurlFetcher::new();
    let frontier = Arc::new(RwLock::new(PriorityFrontier::new()));
    let thread_pool = ThreadPool::new(args.flag_thread_count);
    let _polzatd = PolzatServer::new(args.flag_port, PolzatD::new(frontier.clone()));
    let active_tasks = Arc::new(AtomicUsize::new(0));

    loop {
        //check we should schedule a task
        let clone_active_tasks = active_tasks.clone();
        if clone_active_tasks.load(Ordering::Relaxed) >= args.flag_thread_count {
            std::thread::sleep(Duration::from_millis(500));
            continue
        }

        //read next polzat task from frontier
        let polzat_task;
        {
            let mut frontier = frontier.write().unwrap();
            polzat_task = frontier.pop();
        }

        if polzat_task.is_none() {
            std::thread::sleep(Duration::from_millis(250));
            continue
        }

        //add new job to thread pool
        let thread_frontier = frontier.clone();
        thread_pool.execute(move || {
            //execute polzat task
            let _ = clone_active_tasks.fetch_add(1, Ordering::Relaxed);

            let polzat_task = polzat_task.unwrap();
            let _ = match polzat_task.operation {
                Operation::Crawl => polzat::execute_polzat_crawl(polzat_task, thread_frontier),
                Operation::Scrape => polzat::execute_polzat_scrape(polzat_task),
            };

            let _ = clone_active_tasks.fetch_sub(1, Ordering::Relaxed);
        });
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
                FetcherType::LibCurl,
                LinkExtractorType::Web,
            );
        
        let mut frontier = self.frontier.write().unwrap();
        match frontier.push(polzat_task) {
            Ok(_) => Ok(polzat::create_schedule_task_reply()),
            Err(_) => Err(GrpcError::Other("unable to push task to frontier")),
        }
    }
}
