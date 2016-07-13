extern crate docopt;
extern crate onion_spider;
extern crate rustc_serialize;

use docopt::Docopt;

const USAGE: &'static str = "
Interact with SpiderOnion application

Usage:
    yogi crawl <site> [--ip-address=<ip>] [--port=<port>]
    yogi (-h | --help)

Options:
    -h --help           Show this screen.
    --ip-address=<ip>   IP address of application [default: 127.0.0.1].
    --port=<port>       Port of application [default: 12289].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_crawl: bool,
    arg_site: Option<String>,
    flag_ip_address: String,
    flag_port: i32,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    if args.cmd_crawl {
        //issue crawl request
    }
}
