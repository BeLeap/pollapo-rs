use clap::Subcommand;

use self::{add::AddArgs, install::InstallArgs};

pub mod add;
pub mod install;

#[derive(Subcommand, Debug)]
pub enum Command {
    Add(AddArgs),
    Install(InstallArgs),
}
