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
    let cli = Cli::parse();

    match &cli.subcommand {
        Command::Add(_) => cmds::install::action(),
        Command::Install(_) => cmds::install::action(),
    }
}
