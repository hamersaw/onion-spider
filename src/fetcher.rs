use std::io::Error;

use super::PolzatTask;

use curl::easy::Easy;

pub trait Fetcher {
    fn fetch(&self, polzat_task: &PolzatTask) -> Result<String, Error>;
}

/*
 * LibCurlFetcher
 */
pub struct LibCurlFetcher {
}

impl LibCurlFetcher {
    pub fn new() -> LibCurlFetcher {
        LibCurlFetcher {
        }
    }
}

impl Fetcher for LibCurlFetcher {
    fn fetch(&self, polzat_task: &PolzatTask) -> Result<String, Error> {
        let mut buffer = vec!();
        let mut curl_handle = Easy::new();
        {
            //set curl handle parameters
            try!(curl_handle.url(&polzat_task.url));
            try!(curl_handle.follow_location(true));

            //TODO add host header

            //set transfer function
            let mut transfer = curl_handle.transfer();
            try!(transfer.write_function(|data| {
                buffer.extend_from_slice(data);
                Ok(data.len())
            }));

            //submit curl request
            try!(transfer.perform());
        }

        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }
}
