use frontier::Frontier;

use std::io::Error;
use std::process::Command;
use std::sync::Arc;
use std::thread::sleep_ms;

pub trait Fetcher {
    fn start(&self) -> Result<(), Error>;
    fn fetch(&self, site: String) -> Result<(), Error>;
}

pub struct WgetFetcher {
    download_directory: String,
    frontier: Arc<Frontier>,
}

impl WgetFetcher {
    pub fn new(download_directory: String, frontier: Arc<Frontier>) -> WgetFetcher {
        WgetFetcher {
            download_directory: download_directory,
            frontier: frontier,
        }
    }
}

impl Fetcher for WgetFetcher {
    fn start(&self) -> Result<(), Error> {
        loop {
            println!("polling for next site");
            match self.frontier.get_next_site() {
                Some(site) => {
                    self.fetch(site);
                },
                None => sleep_ms(500),
            }
        }
    }

    fn fetch(&self, site: String) -> Result<(), Error> {
        println!("fetching site {}", site);
        Ok(())
    }
}

/*pub fn wget_download(site: String) -> Result<String, Error> {
    println!("downloading site {}", site);

    match Command::new("torsock").arg(format!("wget -r --no-parent http://{}.onion", site)).spawn() {
        Ok(_) => Ok(site),
        Err(e) => Err(e),
    }
}*/
