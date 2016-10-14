use std::io::Error;

use regex::Regex;

const WEB_URL_REGEX: &'static str = "";
const TOR_HIDDEN_SERVICE_URL_REGEX: &'static str = "(http|https)://(.{16}).onion";

pub trait LinkExtractor {
    fn extract(&self, content: &str) -> Result<Vec<String>, Error>;
}

/*
 * WebExtractor Definition
 */
pub struct WebExtractor {
}

impl WebExtractor {
    pub fn new() -> WebExtractor {
        WebExtractor {
        }
    }
}

impl LinkExtractor for WebExtractor {
    fn extract(&self, content: &str) -> Result<Vec<String>, Error> {
        unimplemented!()
    }
}

/*
 * TorHiddenServiceExtractor Definition
 */
pub struct TorHiddenServiceExtractor {
}

impl TorHiddenServiceExtractor {
    pub fn new() -> TorHiddenServiceExtractor {
        TorHiddenServiceExtractor {
        }
    }
}

impl LinkExtractor for TorHiddenServiceExtractor {
    fn extract(&self, content: &str) -> Result<Vec<String>, Error> {
        let onion_regex = Regex::new(TOR_HIDDEN_SERVICE_URL_REGEX).unwrap();
        let urls = onion_regex.captures_iter(content)
                        .map(|x| x.at(2).unwrap().to_owned())
                        .collect::<Vec<String>>();

        Ok(urls)
    }
}

/*
 *BothExtractor Definition
 */
pub struct BothExtractor {
}

impl BothExtractor {
    pub fn new() -> BothExtractor {
        BothExtractor {
        }
    }
}

impl LinkExtractor for BothExtractor {
    fn extract(&self, content: &str) -> Result<Vec<String>, Error> {
        unimplemented!()
    }
}
