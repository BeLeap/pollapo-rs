use clap::Subcommand;

use self::add::AddArgs;

pub mod add;

#[derive(Subcommand, Debug)]
pub enum Command {
    Add(AddArgs),
}
