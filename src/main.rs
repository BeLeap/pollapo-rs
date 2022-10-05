use clap::Parser;
use cmds::Command;

mod cmds;
mod pollapo_yml;
mod utils;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Command,
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        Command::Install(args) => cmds::install::action(&args.config, &args.token, &args.outdir),
    }
}
