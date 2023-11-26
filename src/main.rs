use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli{
    #[command(Subcommand)]
    install_Cmd : install
    
}



fn main() {
    let args = Cli::parse();

}
