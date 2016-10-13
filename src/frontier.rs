use std::collections::{BinaryHeap, HashMap};
use std::io::{Error, ErrorKind};

use super::PolzatTask;

pub trait Frontier {
    fn push(&mut self, polzat_task: PolzatTask) -> Result<(), Error>;
    fn pop(&mut self) -> Option<PolzatTask>;
    fn peek(&mut self) -> Option<&PolzatTask>;
    fn len(&self) -> usize;
}

/*
 * PriorityFrontier Implementation
 */
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

impl Frontier for PriorityFrontier {
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
