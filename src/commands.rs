use clap::{Parser, Subcommand, Arg};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,

}



#[derive(Subcommand, Debug)]
pub enum Commands{
    #[command(arg_required_else_help = true)]
    /// Install Package
    Install {
        #[arg(
            help = "package name (space separated value)",
            value_delimiter = ' ',
            required = true
        )]
        package: Vec<String>,

        #[arg(
            short,
            long,
            help = "Bypass any confirmation",
        )]
        yes_all: bool,


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