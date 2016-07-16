use std::io::Error;
use std::process::Command;

pub trait Fetcher {
    fn fetch(&self, site: String) -> Result<(), Error>;
}

pub struct WgetFetcher {
    download_directory: String,
}

impl WgetFetcher {
    pub fn new(download_directory: String) -> WgetFetcher {
        WgetFetcher {
            download_directory: download_directory,
        }
    }
}

impl Fetcher for WgetFetcher {
    fn fetch(&self, site: String) -> Result<(), Error> {
        unimplemented!();
    }
}

/*pub fn wget_download(site: String) -> Result<String, Error> {
    println!("downloading site {}", site);

    match Command::new("torsock").arg(format!("wget -r --no-parent http://{}.onion", site)).spawn() {
        Ok(_) => Ok(site),
        Err(e) => Err(e),
    }
}*/
