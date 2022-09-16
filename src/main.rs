use clap::Parser;
use cmds::Command;

mod cmds;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Command,
}

fn main() {
    let _cli = Cli::parse();
}
