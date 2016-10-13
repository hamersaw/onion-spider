use std::io::Error;

use super::PolzatTask;

pub trait Fetcher {
    fn fetch(&self, polzat_task: PolzatTask) -> Result<String, Error>;
}

/*
 * LibcurlFetcher
 */
pub struct LibcurlFetcher {
    
}

impl LibcurlFetcher {
    pub fn new() -> LibcurlFetcher {
        LibcurlFetcher {
        }
    }
}

impl Fetcher for LibcurlFetcher {
    fn fetch(&self, polzat_task: PolzatTask) -> Result<String, Error> {
        unimplemented!();
    }
}
