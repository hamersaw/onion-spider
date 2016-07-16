use std::io::Error;

pub trait Frontier {
    fn add_site(&self, site: &str) -> Result<(), Error>;
    fn get_next_site(&self) -> Option<String>;
    fn len(&self) -> u64;
}

pub struct FIFOFrontier {
}

impl FIFOFrontier {
    pub fn new() -> FIFOFrontier {
        FIFOFrontier{}
    }
}

impl Frontier for FIFOFrontier {
    fn add_site(&self, site: &str) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_next_site(&self) -> Option<String> {
        unimplemented!();
    }

    fn len(&self) -> u64 {
        0
    }
}
