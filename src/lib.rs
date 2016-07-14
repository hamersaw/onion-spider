pub mod fetcher;
pub mod frontier;
pub mod link_extractor;

extern crate capnp;
pub mod message_capnp {
        include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

use std::io::Error;

use capnp::message::{Builder, HeapAllocator};

pub fn create_crawl_request_msg(sites: Vec<String>) -> Result<Builder<HeapAllocator>, Error> {
    let mut builder = Builder::new_default();

    {
        let msg = builder.init_root::<message_capnp::onion_spider_message::Builder>();
        let mut crawl_request = msg.get_message_type().init_crawl_request(sites.len() as u32);

        for (i, site) in sites.iter().enumerate() {
            crawl_request.set(i as u32, site);
        }
    }

    Ok(builder)
}

pub fn create_stats_request_msg() -> Result<Builder<HeapAllocator>, Error> {
    let mut builder = Builder::new_default();

    {
        let msg = builder.init_root::<message_capnp::onion_spider_message::Builder>();
        let _ = msg.get_message_type().set_stats_request(());
    }

    Ok(builder)
}
