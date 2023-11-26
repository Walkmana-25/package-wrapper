use clap::Parser;
use commands::Commands;

mod commands;
mod install;
mod libs;


fn main() {
    let args = commands::Cli::parse();

    match &args.command{
        Commands::Install { package, yes_all } => {
            println!("HI");
        }
        _ => {
            println!("HEY");
        }
    }

}
