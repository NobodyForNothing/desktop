use clap::{Parser, Subcommand};
use vcs::git::repo::Repository;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        None => println!("Unrecognized command"),
        Some(Commands::Init {path}) => {
            Repository::init(path.parse().unwrap()).unwrap();
        }
    }

}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        /// Where to create the repository.
        #[arg(default_value = ".")]
        path: String,
    },
}