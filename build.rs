extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["schema/message.capnp"]).unwrap();
    println!("Succesfully compiled capnproto files");
}
