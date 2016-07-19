use std::io::Error;

pub trait LinkExtractor {
    fn extract(&self, site: &str) -> Result<Vec<String>, Error>;
}

pub struct IterativeExtractor {
    download_directory: String,
}

impl IterativeExtractor {
    pub fn new(download_directory: String) -> IterativeExtractor {
        IterativeExtractor {
            download_directory: download_directory,
        }
    }
}

impl LinkExtractor for IterativeExtractor {
    fn extract(&self, site: &str) -> Result<Vec<String>, Error> {
        println!("TODO extract link from site {}", site);
        Ok(Vec::new())
    }
}

