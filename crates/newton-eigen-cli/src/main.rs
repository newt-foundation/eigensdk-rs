use clap::Parser;
use newton_cli::{args::Args, execute_command};

fn main() {
    let args = Args::parse();
    execute_command(args.command).unwrap();
}
