use clap::Parser;

mod commands;
mod libs;



fn main() {
    let args = commands::Cli::parse();


    match args.command {
        commands::Commands::Install { remote } => {
            println!("Cloning {remote}");
        }
        _ => {
            println!("HI");
        }

    }



}
