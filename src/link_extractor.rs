use std::fs::File;
use std::io::{Error, Read};
use std::str::from_utf8;

use glob::glob;
use regex::Regex;

pub trait LinkExtractor {
    fn extract(&self, site: &str) -> Result<Vec<String>, Error>;
}

pub struct IterativeExtractor {
    download_directory: String,
    re: Regex,
}

impl IterativeExtractor {
    pub fn new(download_directory: String) -> IterativeExtractor {
        IterativeExtractor {
            download_directory: download_directory,
            re: Regex::new("(http|https)://(.{16}).onion").unwrap(),
        }
    }
}

impl LinkExtractor for IterativeExtractor {
    fn extract(&self, site: &str) -> Result<Vec<String>, Error> {
        let mut sites = Vec::new();

        let iter = match glob(&format!("{}/{}.onion/**/*", self.download_directory, site)) {
            Ok(iter) => iter,
            Err(e) => panic!("unable to parse glob pattern: {}", e),
        };
        
        for entry in iter {
            match entry {
                Ok(path) => {
                    if !path.is_file() {
                        continue
                    }
                    println!("processing file {}", path.display());

                    let mut buf = Vec::new();
                    let mut f = try!(File::open(format!("{}", path.display())));
                    try!(f.read_to_end(&mut buf));
                    let file_str = match from_utf8(&buf) {
                        Ok(file_str) => file_str,
                        Err(e) => {
                            println!("error parsing file from utf8: {}", e);
                            continue
                        },
                    };
                    
                    for onion_addr in self.re.captures_iter(file_str) {
                        sites.push(onion_addr.at(2).unwrap().to_string());
                    }
                },
                Err(e) => panic!("error retreiving entry: {}", e),
            }
        }

        Ok(sites)
    }
}
