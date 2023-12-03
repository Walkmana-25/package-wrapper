use anyhow::Error;
use clap::Parser;
use commands::Commands;
use libs::run_cmd_root;

use crate::libs::run_cmd;

mod commands;
mod install;
mod libs;


fn main() -> Result<(), Error> {
    let args = commands::Cli::parse();
    let package_manager = libs::select_package_manager()?;

    match &args.command{
        Commands::Install { package, yes_all } => {
            let cmd = install::gen_cmd(
                    &package_manager, Some(package), Some(*yes_all), install::ModeSelect::Install
                )?;
            println!("Run `{}`", cmd.join(" "));

            run_cmd_root(cmd)?;

            Ok(())
        },
        Commands::Remove { package, yes_all } => {
            let cmd = install::gen_cmd(
                    &package_manager, Some(package), Some(*yes_all), install::ModeSelect::Remove
                )?;
            println!("Run `{}`", cmd.join(" "));

            run_cmd_root(cmd)?;

            Ok(())
        },
        Commands::Update { yes_all } => {
            let cmd = install::gen_cmd(
                    &package_manager, None, Some(*yes_all), install::ModeSelect::Update
                )?;
            println!("Run `{}`", cmd.join(" "));

            run_cmd_root(cmd)?;

            Ok(())

        },
        Commands::Search {package} => {
            let cmd = install::gen_cmd(
                &package_manager, Some(package), None, install::ModeSelect::Search
            )?;
            println!("Run `{}`", cmd.join(" "));

            run_cmd(cmd)?;

            Ok(())
        },
        Commands::List => {
            let cmd = install::gen_cmd(
                &package_manager, None, None, install::ModeSelect::List
            )?;
            println!("Run `{}`", cmd.join(" "));

            run_cmd(cmd)?;

            Ok(())

        }
    }

}
