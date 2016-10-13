use std::collections::{BinaryHeap, HashMap};
use std::io::{Error, ErrorKind};
use std::sync::RwLock;

use super::PolzatTask;

pub trait Frontier {
    fn add_site(&self, site: &str) -> Result<(), Error>;
    fn get_next_site(&self) -> Option<String>;
    fn len(&self) -> u64;
}

/*
 * FIFOFrontier implementation
 */
pub struct FIFOFrontier {
    buffer: RwLock<Vec<String>>,
    crawled: RwLock<Vec<String>>,
}

impl FIFOFrontier {
    pub fn new() -> FIFOFrontier {
        FIFOFrontier {
            buffer: RwLock::new(Vec::new()),
            crawled: RwLock::new(Vec::new()),
        }
    }
}

impl Frontier for FIFOFrontier {
    fn add_site(&self, site: &str) -> Result<(), Error> {
        //search current buffer
        {
            let read_buffer = self.buffer.read().unwrap();
            for s in read_buffer.iter() {
                if s == site {
                    return Ok(());
                }
            }
        }

        //search crawled
        {
            let read_crawled = self.crawled.read().unwrap();
            for s in read_crawled.iter() {
                if s == site {
                    return Ok(());
                }
            }
        }

        //add to buffer
        let mut write_buffer = self.buffer.write().unwrap();
        write_buffer.push(site.to_string());
        Ok(())
    }

    fn get_next_site(&self) -> Option<String> {
        let mut write_buffer = self.buffer.write().unwrap();
        let site = match write_buffer.pop() {
            Some(site) => site,
            None => return None,
        };

        let mut write_crawled = self.crawled.write().unwrap();
        write_crawled.push(site.clone());
        Some(site)
    }

    fn len(&self) -> u64 {
        let read_buffer = self.buffer.read().unwrap();
        read_buffer.len() as u64
    }
}

/*
 * PriorityFrontier Implementation
 */
pub trait FrontierV2 {
    fn push(&mut self, polzat_task: PolzatTask) -> Result<(), Error>;
    fn pop(&mut self) -> Option<PolzatTask>;
    fn peek(&mut self) -> Option<&PolzatTask>;
    fn len(&self) -> usize;
}

pub struct PriorityFrontier {
    queue: BinaryHeap<PolzatTask>,
    seen: HashMap<u32, Vec<String>>,
}

impl PriorityFrontier {
    pub fn new() -> PriorityFrontier {
        PriorityFrontier {
            queue: BinaryHeap::new(),
            seen: HashMap::new(),
        }
    }
}

impl FrontierV2 for PriorityFrontier {
    fn push(&mut self, polzat_task: PolzatTask) -> Result<(), Error> {
        //check for existance in 'seen'
        let mut urls = self.seen.entry(polzat_task.execution_id).or_insert(vec!());
        if contains(urls, &polzat_task.url) {
            return Err(Error::new(ErrorKind::Other, format!("execution_id '{}' has already processed url '{}'", polzat_task.execution_id, polzat_task.url)))
        }

        urls.push(polzat_task.url.to_owned());

        //add polzat task to queue
        self.queue.push(polzat_task);
        
        Ok(())
    }

    fn pop(&mut self) -> Option<PolzatTask> {
        self.queue.pop()
    }

    fn peek(&mut self) -> Option<&PolzatTask> {
        self.queue.peek()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}

fn contains(vec: &Vec<String>, value: &str) -> bool {
    for v in vec {
        if v == value {
            return true
        }
    }

    return false
}
