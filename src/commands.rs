use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,

}



#[derive(Subcommand)]
pub enum Commands{
    #[command(arg_required_else_help = true)]
    /// Install Package
    Install {
        remote: String,
    },
    /// Remove Package
    Remove {
        remote: String,
    },
    /// Search Repository Package
    Search {
        remote: String,
    },
    /// Show installed Package
    List {
        remote: String,
    }





}