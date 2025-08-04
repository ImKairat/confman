use clap::Parser;
use crate::cli::models::{Args, Commands};


pub fn get_args() -> Result<Commands, std::io::Error> {
    let args = Args::parse().command;
    return Ok(args);
}
