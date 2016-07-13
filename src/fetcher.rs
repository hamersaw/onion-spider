use std::io::Error;
use std::process::Command;

trait Fetcher {
    fn fetch(site: String) -> Result<(), Error>;
}

struct WgetFetcher {
    download_directory: String,
}

impl Fetcher for WgetFetcher {
    fn fetch(site: String) -> Result<(), Error> {
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
