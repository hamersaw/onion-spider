pub mod fetcher;
pub mod frontier;
pub mod link_extractor;
pub mod polzat_pb;
pub mod polzat_pb_grpc;

extern crate curl;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate regex;

use std::io::{Error, ErrorKind};
use std::cmp::{Ordering, PartialOrd};
use std::sync::{Arc, RwLock};

use fetcher::{Fetcher, LibCurlFetcher};
use frontier::PriorityFrontier;
use link_extractor::{BothExtractor, LinkExtractor, TorHiddenServiceExtractor, WebExtractor};
use polzat_pb::{ScheduleTaskReply, ScheduleTaskRequest, ScheduleTaskRequest_UrlType, ScheduleTaskRequest_Operation};

/*
 * PolzatTask definition
 */
#[derive(Debug)]
pub enum UrlType {
    Web,
    TorHiddenService,
}

#[derive(Debug)]
pub enum Operation {
    Crawl,
    Scrape,
}

#[derive(Debug)]
pub enum FetcherType {
    LibCurl,
    Empty,
}

#[derive(Debug)]
pub enum LinkExtractorType {
    Web,
    TorHiddenService,
    Both,
    Empty,
}

#[derive(Debug)]
pub struct PolzatTask {
    pub execution_id: u32,
    pub priority: u8,
    pub url: String,
    pub url_type: UrlType,
    pub operation: Operation,
    pub fetcher_type: FetcherType,
    pub link_extractor_type: LinkExtractorType,
}

impl PolzatTask {
    pub fn new(execution_id: u32, priority: u8, url: String, url_type: UrlType, operation: Operation, fetcher_type: FetcherType, link_extractor_type: LinkExtractorType) -> PolzatTask {
        PolzatTask {
            execution_id: execution_id,
            priority: priority,
            url: url,
            url_type: url_type,
            operation: operation,
            fetcher_type: fetcher_type,
            link_extractor_type: link_extractor_type,
        }
    }
}

impl PartialEq for PolzatTask {
    fn eq(&self, other: &PolzatTask) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PolzatTask {}

impl Ord for PolzatTask {
    fn cmp(&self, other: &PolzatTask) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PolzatTask {
    fn partial_cmp(&self, other: &PolzatTask) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
 * Execute Polzat Task
 */
pub fn execute_polzat_crawl(polzat_task: PolzatTask, frontier: Arc<RwLock<PriorityFrontier>>) -> Result<(), Error> {
    let fetcher: Box<Fetcher> = match polzat_task.fetcher_type {
        FetcherType::LibCurl => Box::new(LibCurlFetcher::new()) as Box<Fetcher>,
        _ => return Err(Error::new(ErrorKind::Other, "Unable to execute polzat task with empty fetcher_type")),
    };

    let link_extractor: Box<LinkExtractor> = match polzat_task.link_extractor_type {
        LinkExtractorType::Web => Box::new(WebExtractor::new()) as Box<LinkExtractor>,
        LinkExtractorType::TorHiddenService => Box::new(TorHiddenServiceExtractor::new()) as Box<LinkExtractor>,
        LinkExtractorType::Both => Box::new(BothExtractor::new()) as Box<LinkExtractor>,
        _ => return Err(Error::new(ErrorKind::Other, "Unable to execute polzat task with empty link_extractor_type")),
    };

    let response = try!(fetcher.fetch(&polzat_task));
    let urls = try!(link_extractor.extract(&response));

    Ok(())
}

pub fn execute_polzat_scrape(polzat_task: PolzatTask) -> Result<(), Error> {
    unimplemented!()
}

/*
 * Create Protobuf Messages
 */
pub fn create_schedule_task_reply() -> ScheduleTaskReply {
    ScheduleTaskReply::new()
}

pub fn create_schedule_task_request(execution_id: u32, priority: u8, url: &str, url_type: UrlType, operation: Operation) -> ScheduleTaskRequest {
    let mut request = ScheduleTaskRequest::new();
    request.set_execution_id(execution_id);
    request.set_priority(priority as u32);
    request.set_url(url.to_owned());

    match url_type {
        UrlType::Web => request.set_url_type(ScheduleTaskRequest_UrlType::Web),
        UrlType::TorHiddenService => request.set_url_type(ScheduleTaskRequest_UrlType::TorHiddenService), 
    }

    match operation {
        Operation::Crawl => request.set_operation(ScheduleTaskRequest_Operation::Crawl),
        Operation::Scrape => request.set_operation(ScheduleTaskRequest_Operation::Scrape),
    }

    request
}
