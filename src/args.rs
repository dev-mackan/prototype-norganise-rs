use std::fs::File;

use clap::Parser;
use log::{debug, info};
use simplelog::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    debug: bool,
}

pub fn parse() {
    let args = Args::parse();
    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();

    if args.debug {
        // Log everything to file
        loggers.push(WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("debug.log").unwrap(),
        ));

        // Errors also to stderr
        loggers.push(TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Stderr,
            ColorChoice::Auto,
        ));
    } else {
        // Only log errors to stderr (no file logging)
        loggers.push(TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Stderr,
            ColorChoice::Auto,
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}
