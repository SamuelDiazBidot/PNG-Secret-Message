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
        SubCommand::Encode(args) => {
            encode(args)?;
        },
        SubCommand::Decode(args) => {
            println!("{}", decode(args)?);
        },
        SubCommand::Remove(args) => {
            remove(args)?;
        },
        SubCommand::Print(args) => {
            print_chunks(args)?;
        }
    }

    Ok(())
}