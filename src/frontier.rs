use std::io::Error;
use std::sync::RwLock;

pub trait Frontier {
    fn add_site(&self, site: &str) -> Result<(), Error>;
    fn get_next_site(&self) -> Option<String>;
    fn len(&self) -> u64;
}

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
