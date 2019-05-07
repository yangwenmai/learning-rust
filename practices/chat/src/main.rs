use sonr::prelude::*;
use sonr::net::stream::Stream;
use sonr::errors::Result;
use sonr_extras::tcp_listener;

mod connections;
use connections::{User, Connections};

fn main() -> Result<()> {
    System::init()?;

    let listener = tcp_listener("0.0.0.0:4578")?
        .map(|s| {
            let stream = Stream::new(s).unwrap();
            User::new(stream)
        });

    let connections = Connections::new();

    let run = listener.chain(connections);

    System::start(run)?;
    Ok(())
}