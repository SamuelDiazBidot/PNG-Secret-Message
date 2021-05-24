mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Clap;
use args::*;
use commands::{encode, decode, remove, print_chunks};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Encode(encode) => {
            println!("{:?} {}", encode.file, encode.message)
        },
        SubCommand::Decode(decode) => {
            println!("{:?} {}",decode.file, decode.chunk_type)
        },
        SubCommand::Remove(remove) => {
            println!("{:?} {}", remove.file, remove.chunk_type)
        },
        SubCommand::Print(print) => {
            print_chunks(print)?;
        }
    }

    Ok(())
}