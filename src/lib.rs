pub mod fetcher;
pub mod frontier;
pub mod link_extractor;
pub mod polzat_pb;
pub mod polzat_pb_grpc;

extern crate glob;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate regex;

use std::cmp::{Ordering, PartialOrd};
use std::io::Error;

use polzat_pb::{ScheduleTaskReply, ScheduleTaskRequest, ScheduleTaskRequest_UrlType, ScheduleTaskRequest_Operation};

/*
 * PolzatTask definition
 */
pub enum UrlType {
    Web,
    TorHiddenService,
}

pub enum Operation {
    Crawl,
    Scrape,
}

pub struct PolzatTask {
    execution_id: u32,
    priority: u8,
    url: String,
    url_type: UrlType,
    operation: Operation,
}

impl PolzatTask {
    pub fn new(execution_id: u32, priority: u8, url: String, url_type: UrlType, operation: Operation) -> PolzatTask {
        PolzatTask {
            execution_id: execution_id,
            priority: priority,
            url: url,
            url_type: url_type,
            operation: operation,
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
