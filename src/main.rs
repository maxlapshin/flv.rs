extern crate futures;
extern crate tokio_core;
extern crate getopts;
pub mod flv;
pub mod frame;

// use futures::{Future, Stream};
// use tokio_core::io::{copy, Io};
// use tokio_core::net::TcpListener;
// use tokio_core::reactor::Core;
use getopts::Options;
use std::env;
use flv::FlvStream;


fn main() {
    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("f", "flv", "dump input flv file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };    
    if matches.opt_present("f") {
        dump_flv(matches.opt_str("f").unwrap().clone());
        return;
    }
    // Create the event loop that will drive this server
    // let mut core = Core::new().unwrap();
    // let handle = core.handle();

    // // Bind the server's socket
    // let addr = "127.0.0.1:12345".parse().unwrap();
    // let sock = TcpListener::bind(&addr, &handle).unwrap();

    // // Pull out a stream of sockets for incoming connections
    // let server = sock.incoming().for_each(|(sock, _)| {
    //     // Split up the reading and writing parts of the
    //     // socket
    //     let (reader, writer) = sock.split();

    //     // A future that echos the data and returns how
    //     // many bytes were copied...
    //     let bytes_copied = copy(reader, writer);

    //     // ... after which we'll print what happened
    //     let handle_conn = bytes_copied.map(|amt| {
    //         println!("wrote {} bytes", amt)
    //     }).map_err(|err| {
    //         println!("IO error {:?}", err)
    //     });

    //     // Spawn the future as a concurrent task
    //     handle.spawn(handle_conn);

    //     Ok(())
    // });

    // // Spin up the server on the event loop
    // core.run(server).unwrap();
}



fn dump_flv(path: String) {
    let mut f = FlvStream::new(path);
    loop {
        let frame = f.next();
        println!("{:?}", frame);
    }
}
