use std::io::Error;

pub trait Frontier {
    fn add_site(&mut self, site: &str) -> Result<(), Error>;
    fn get_next_site(&mut self) -> Option<String>;
    fn len(&self) -> u64;
}

pub struct FIFOFrontier {
    buffer: Vec<String>,
    crawled: Vec<String>,
}

impl FIFOFrontier {
    pub fn new() -> FIFOFrontier {
        FIFOFrontier {
            buffer: Vec::new(),
            crawled: Vec::new(),
        }
    }
}

impl Frontier for FIFOFrontier {
    fn add_site(&mut self, site: &str) -> Result<(), Error> {
        //search current buffer
        for s in self.buffer.iter() {
            if s == site {
                return Ok(());
            }
        }

        //search crawled
        for s in self.crawled.iter() {
            if s == site {
                return Ok(());
            }
        }

        //add to buffer
        self.buffer.push(site.to_string());
        Ok(())
    }

    fn get_next_site(&mut self) -> Option<String> {
        let site = match self.buffer.pop() {
            Some(site) => site,
            None => return None,
        };

        self.crawled.push(site.clone());
        Some(site)
    }

    fn len(&self) -> u64 {
        self.buffer.len() as u64
    }
}
