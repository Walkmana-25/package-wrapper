use anyhow::Error;
use clap::Parser;
use commands::Commands;
use libs::run_cmd_root;

mod commands;
mod install;
mod libs;


fn main() -> Result<(), Error> {
    let args = commands::Cli::parse();
    let package_manager = libs::select_package_manager()?;

    match &args.command{
        Commands::Install { package, yes_all } => {
            let cmd = install::gen_cmd(
                    &package_manager, package, *yes_all, install::ModeSelect::Install
                )?;
            println!("Run `{}`", cmd.join(" "));

            run_cmd_root(cmd)?;

            Ok(())
        }
        _ => {
            println!("HEY");
            Ok(())
        }
    }

}
