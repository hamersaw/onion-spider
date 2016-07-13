extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["schema/spider_onion_message.capnp"]).unwrap();
    println!("Succesfully compiled capnproto files");
}
