use args::Args;
use clap::Parser;
use crate::args::Commands::{Decode, Encode, Print, Remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    //todo!()
    {
        let args = Args::parse();
        match &args.command {
            Encode {path, chunk_type, message} => {
                println!("Called encode with path: {}, chunk_t: {}, and message: {}", path, chunk_type, message);
            },
            Decode {path, chunk_type} => (),
            Remove {path, chunk_type} => (),
            Print {path} => ()
        }
    }
    Ok(())
}
