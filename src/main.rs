use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
