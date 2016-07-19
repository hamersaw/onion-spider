use frontier::Frontier;
use link_extractor::LinkExtractor;

use std::io::Error;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub trait Fetcher {
    fn start(&self) -> Result<(), Error>;
    fn fetch(&self, site: &str) -> Result<(), Error>;
}

pub struct WgetFetcher {
    download_directory: String,
    frontier: Arc<Frontier>,
    link_extractor: Box<LinkExtractor>,
}

impl WgetFetcher {
    pub fn new(download_directory: String, frontier: Arc<Frontier>, link_extractor: Box<LinkExtractor>) -> WgetFetcher {
        WgetFetcher {
            download_directory: download_directory,
            frontier: frontier,
            link_extractor: link_extractor,
        }
    }
}

impl Fetcher for WgetFetcher {
    fn start(&self) -> Result<(), Error> {
        loop {
            match self.frontier.get_next_site() {
                Some(site) => {
                    //fetch site
                    try!(self.fetch(&site));

                    //extract links and add to frontier
                    let sites = try!(self.link_extractor.extract(&site));
                    for site in sites {
                        try!(self.frontier.add_site(&site));
                    }
                },
                None => sleep(Duration::from_millis(500)),
            }
        }
    }

    fn fetch(&self, site: &str) -> Result<(), Error> {
        match Command::new("torsocks")
                    .args(&["wget", "-r", "--no-parent", "-P", &self.download_directory, &format!("http://{}.onion", site)])
                    .stdout(Stdio::null())
                    .output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        }

        Ok(())
    }
}
