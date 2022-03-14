extern crate core;

// use crate::args::Cli;
use crate::args::PngMeArgs::{self, Decode, Encode, Print, Remove};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();

    match args {
        Encode(args) => commands::encode(args)?,
        Decode(args) => commands::decode(args)?,
        Remove(args) => commands::remove(args)?,
        Print(args) => commands::print(args)?,
    };

    Ok(())
}
