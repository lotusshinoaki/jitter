use clap::Parser;
use rand::prelude::*;
use std::process::{exit, Command};
use std::{thread, time};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = false)]
    quiet: bool,
    #[clap(short, long = "min_delay", value_parser, default_value_t = 0u32)]
    min_delay_s: u32,
    #[clap(value_parser)]
    max_delay_s: u32,
    #[clap(value_parser)]
    utility: String,
    #[clap(value_parser)]
    argument: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let min_delay_ms = (args.min_delay_s as u64) * 1000u64;
    let max_delay_ms = (args.max_delay_s as u64) * 1000u64;

    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(min_delay_ms..max_delay_ms);
    if !args.quiet {
        let delay_s = (delay_ms as f64) / 1000f64;
        println!("Wait {} seconds", delay_s);
    }
    thread::sleep(time::Duration::from_millis(delay_ms));

    let mut command = Command::new(&args.utility);
    for a in &args.argument {
        command.arg(a);
    }

    match command.status() {
        Ok(status) => match status.code() {
            Some(code) => exit(code),
            None => exit(1),
        },
        Err(e) => {
            if !args.quiet {
                eprintln!("Error: Could not spawn child process. {}", e);
            }
            exit(1);
        }
    }
}
