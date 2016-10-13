extern crate docopt;
extern crate polzat;
extern crate rustc_serialize;

use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;

use docopt::Docopt;
use polzat::{UrlType, Operation};
use polzat::polzat_pb_grpc::{Polzat, PolzatClient};

const USAGE: &'static str = "
Interact with PolzatD application

Usage:
    polzat crawl <url> [--ip-address=<ip>] [--port=<port>]
    polzat scrape <url> [--ip-address=<ip>] [--port=<port>]
    polzat (-h | --help)

Options:
    -h --help           Show this screen.
    --ip-address=<ip>   IP address of application [default: 127.0.0.1].
    --port=<port>       Port of application [default: 12289].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_crawl: bool,
    cmd_scrape: bool,
    arg_url: String,
    flag_ip_address: String,
    flag_port: u16,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let client = PolzatClient::new(&args.flag_ip_address, args.flag_port, false).unwrap();

    if args.cmd_crawl {
        let request = polzat::create_schedule_task_request(0, 0, &args.arg_url, UrlType::Web, Operation::Crawl);
        let response = client.ScheduleTask(request);

        println!("response: {:?}", response);
    } else if args.cmd_scrape {
        let request = polzat::create_schedule_task_request(0, 0, &args.arg_url, UrlType::Web, Operation::Scrape);
        let response = client.ScheduleTask(request);

        println!("response: {:?}", response);
    }
}
