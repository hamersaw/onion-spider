use std::io::Error;

use std::collections::HashMap;

use regex::Regex;

use super::UrlType;

const WEB_URL_REGEX: &'static str = "(?:http|https)://www\\.([a-zA-Z0-9_\\.]*)(\\.com|\\.edu|\\.gov|\\.net|\\.org)(/[a-zA-Z0-9_?=&\\.]*)*";
const TOR_HIDDEN_SERVICE_URL_REGEX: &'static str = "(http|https)://(?:.{16}).onion(/[/a-zA-Z0-9_?=&\\.]*)*";

pub trait LinkExtractor {
    fn extract(&self, content: &str) -> Result<Vec<String>, Error>;
    fn extract_map(&self, content: &str) -> Result<HashMap<UrlType, Vec<String>>, Error>;
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
        let web_regex = Regex::new(WEB_URL_REGEX).unwrap();
        Ok(web_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>())
    }

    fn extract_map(&self, content: &str) -> Result<HashMap<UrlType, Vec<String>>, Error> {
        let web_regex = Regex::new(WEB_URL_REGEX).unwrap();
        let web_urls = web_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>();

        let mut map = HashMap::new();
        if web_urls.len() != 0 {
            let _ = map.insert(UrlType::Web, web_urls);
        }
        Ok(map)
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
        let tor_regex = Regex::new(TOR_HIDDEN_SERVICE_URL_REGEX).unwrap();
        Ok(tor_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>())
    }

    fn extract_map(&self, content: &str) -> Result<HashMap<UrlType, Vec<String>>, Error> {
        let tor_regex = Regex::new(TOR_HIDDEN_SERVICE_URL_REGEX).unwrap();
        let tor_urls = tor_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>();

        let mut map = HashMap::new();
        if tor_urls.len() != 0 {
            let _ = map.insert(UrlType::TorHiddenService, tor_urls);
        }
        Ok(map)
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
        let web_regex = Regex::new(WEB_URL_REGEX).unwrap();
        let mut web_urls = web_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>();

        let tor_regex = Regex::new(WEB_URL_REGEX).unwrap();
        let mut tor_urls = tor_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>();

        web_urls.append(&mut tor_urls);
        Ok(web_urls)
    }

    fn extract_map(&self, content: &str) -> Result<HashMap<UrlType, Vec<String>>, Error> {
        let web_regex = Regex::new(WEB_URL_REGEX).unwrap();
        let web_urls = web_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>();

        let tor_regex = Regex::new(WEB_URL_REGEX).unwrap();
        let tor_urls = tor_regex.find_iter(content)
                    .map(|(start, end)| content[start..end].to_owned())
                    .collect::<Vec<String>>();

        let mut map = HashMap::new();
        if web_urls.len() != 0 {
            let _ = map.insert(UrlType::Web, web_urls);
        }

        if tor_urls.len() != 0 {
            let _ = map.insert(UrlType::TorHiddenService, tor_urls);
        }

        Ok(map)
    }
}
