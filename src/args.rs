use std::path::PathBuf;
use clap::{AppSettings, Clap};

/// Encode and Decode messages into a PNG file.
#[derive(Clap)]
#[clap(version = "1.0", author = "Samuel Diaz Bidot <samuel.diaz9@upr.edu>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

/// Encode a message into the PNG file
#[derive(Clap)]
pub struct Encode {
    pub file: PathBuf,
    pub chunk_type: String,
    pub message: String,
}

/// Decode a message from a PNG file
#[derive(Clap)]
pub struct Decode {
    pub file: PathBuf,
    pub chunk_type: String,
}

/// Remove a chunk from a PNG file
#[derive(Clap)]
pub struct Remove {
    pub file: PathBuf,
    pub chunk_type: String,
}
/// Print all the chunks in a PNG file
#[derive(Clap)]
pub struct Print {
    pub file: PathBuf,
}