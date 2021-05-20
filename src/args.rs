use std::path::PathBuf;

use clap::{AppSettings, Clap};

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
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

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct Encode {
    pub file: PathBuf,
    pub chunk_type: String,
    pub message: String,
}

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct Decode{
    pub file: PathBuf,
    pub chunk_type: String,
}

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct Remove{
    pub file: PathBuf,
    pub chunk_type: String,
}
/// A subcommand for controlling testing
#[derive(Clap)]
pub struct Print{
    pub file: PathBuf,
}