use std::collections::HashMap;
use regex::Regex;

pub trait UrlValidator {
    fn is_valid(&mut self, url: &str) -> bool;
}

/*
 * RobotsValidator
 */
pub struct RobotsValidator {
    robots_cache: HashMap<String, Regex>,
}

impl RobotsValidator {
    pub fn new() -> RobotsValidator {
        RobotsValidator {
            robots_cache: HashMap::new(),
        }
    }
}

impl UrlValidator for RobotsValidator {
    fn is_valid(&mut self, url: &str) -> bool {
        //parse out domain and extension of url
        let url_clean = url.replace("https://", "").replace("http://", "");

        let index = url_clean.find("/");
        let domain = match index {
            Some(index) => &url_clean[..index],
            None => &url_clean,
        };

        let extension = match index {
            Some(index) => &url_clean[index..],
            None => "*",
        };

        //retrieve regex from map
        let decline_regex = self.robots_cache.entry(domain.to_owned()).or_insert_with(|| {
            //fetch and parse robots.txt for domain
            Regex::new("a^").unwrap() //match nothing
        });

        !decline_regex.is_match(extension)
    }
}
