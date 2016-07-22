pub mod fetcher;
pub mod frontier;
pub mod link_extractor;

extern crate capnp;
extern crate glob;
extern crate regex;

pub mod message_capnp {
        include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

use std::io::Error;

use capnp::message::{Builder, HeapAllocator};

pub fn create_crawl_request(sites: Vec<String>) -> Result<Builder<HeapAllocator>, Error> {
    let mut builder = Builder::new_default();

    {
        let msg = builder.init_root::<message_capnp::onion_spider_message::Builder>();
        let mut crawl_request = msg.get_message_type().init_crawl_request(sites.len() as u32);

        for (i, site) in sites.iter().enumerate() {
            crawl_request.borrow().set(i as u32, &site);
        }
    }

    Ok(builder)
}

pub fn create_stats_request() -> Result<Builder<HeapAllocator>, Error> {
    let mut builder = Builder::new_default();

    {
        let msg = builder.init_root::<message_capnp::onion_spider_message::Builder>();
        let _ = msg.get_message_type().set_stats_request(());
    }

    Ok(builder)
}

pub fn create_stats_reply(frontier_size: u64) -> Result<Builder<HeapAllocator>, Error> {
    let mut builder = Builder::new_default();

    {
        let msg = builder.init_root::<message_capnp::onion_spider_message::Builder>();
        let mut stats_reply = msg.get_message_type().init_stats_reply();

        stats_reply.set_frontier_size(frontier_size);
    }

    Ok(builder)
}
