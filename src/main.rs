use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::sync::mpsc;
use std::thread;

static THREAD_COUNT: i32 = 10;

fn main() {
    let mut site_chans = Vec::new();
    let (done_tx, done_rx) = mpsc::channel();

    //start up all of the threads
    for id in 0..THREAD_COUNT {
        let (site_tx, site_rx) = mpsc::channel();
        site_chans.push(site_tx);
        let thread_done_tx = done_tx.clone();

        thread::spawn(move || {
            println!("started thread {}", id);

            while true {
                let site = site_rx.recv().unwrap();

                println!("thread {} working on site {}", id, site);
                thread_done_tx.send(id);
            }
        });
    }

    //read in onion addresses
    let mut site_buffer: Vec<String> = Vec::new();
    let file = File::open("examples/sites.txt").unwrap();
    let reader = BufReader::new(file);

    for site in reader.lines() {
        site_buffer.push(site.unwrap());
    }

    //push onion addresses to work channel
    let (mut index, mut active_sites) = (0, 0);
    while site_buffer.len() != 0 {
        let site = site_buffer.pop().unwrap();

        site_chans[index].send(site);
        index = (index + 1) % site_chans.len();
        active_sites += 1;
    }


    while active_sites > 0 {
        let chan_id = done_rx.recv().unwrap();
        
        println!("completed:{}", chan_id);
        active_sites -= 1;
    }
}

/*fn main() {
    //connect to site through proxy
    let proxy_addr = SocketAddr::from_str("127.0.0.1:9050").unwrap();

    let (mut onion_addrs, mut crawled_addrs) = (vec!("xmh57jrzrnw6insl".to_string()), Vec::new());
    while onion_addrs.len() != 0 {
        let onion_addr = onion_addrs.pop().unwrap();
        let target_addr = TargetAddr::Domain(format!("{}.onion", onion_addr).to_string(), 80);

        match crawl(&proxy_addr, target_addr) {
            Ok(found_addrs) => {
                for addr in found_addrs {
                    println!("found: {}", addr);
                    onion_addrs.push(addr);
                }
            },
            Err(_) => println!("error crawling {}", onion_addr),
        }

        crawled_addrs.push(onion_addr);
    }

    println!("crawled {} addresses", crawled_addrs.len());
}

fn crawl(proxy_addr: &SocketAddr, target_addr: TargetAddr) -> Result<Vec<String>, Error> {
    //open stream
    let mut stream = try!(Socks5Stream::connect(proxy_addr, target_addr));

    //write HTTP GET
    try!(stream.write_all(b"GET / HTTP/1.0\r\n\r\n"));

    //read response
    let mut buf = Vec::new();
    let _ = try!(stream.read_to_end(&mut buf));
    let response_str = try!(std::str::from_utf8(&buf));

    //search for .onion addresses in response
    let re = Regex::new("(http|https)://(.{16}).onion").unwrap();
    let mut found_addrs = Vec::new();
    for onion_addr in re.captures_iter(response_str) {
        found_addrs.push(onion_addr.at(2).unwrap().to_string());
    }

    Ok(found_addrs)
}*/
