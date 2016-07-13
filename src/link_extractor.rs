use std::io::Error;

trait LinkExtractor {
    fn extract(site: String) -> Result<Vec<String>, Error>;
}

struct IterativeExtractor {
    download_directory: String,
}

impl LinkExtractor for IterativeExtractor {
    fn extract(site: String) -> Result<Vec<String>, Error> {
        unimplemented!();
    }
}

