/**
 * This is the simplest implementation of an Otto Engine, which keeps everything
 * only in memory
 */

#[deny(unsafe_code)]

extern crate eventbus_inmemory;

use smol;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();

    let addr = "127.0.0.1:8105".to_string();
    smol::run(eventbus_inmemory::run_server(addr))
}

