use clap::Subcommand;

use self::install::InstallArgs;

pub mod install;

#[derive(Subcommand, Debug)]
pub enum Command {
    Install(InstallArgs),
}
