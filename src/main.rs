use clap::Parser;

mod commands;



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
