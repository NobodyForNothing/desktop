use std::io;
use std::io::Write;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use vcs::git::objects::{GitObject, GitObjectType};
use vcs::git::repo::{Repository, RepositoryLoadError};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        None => println!("Unrecognized command"),
        Some(Commands::Init {path}) => {
            Repository::init(path.parse().unwrap()).unwrap();
        }
        Some(Commands::CatFile { obj_type, object }) => {
            let repo = Repository::new(PathBuf::from(cli.repo_path), None);
            match repo {
                Err(err) => eprintln!("{:?}", err),
                Ok(repo) => {
                    let obj = repo.object_read(repo.object_find(object, obj_type, true));
                    match obj {
                        None => println!("No object found"),
                        Some(obj) =>{
                            io::stdout().lock().write(&*obj.serialize()).unwrap();
                            io::stdout().lock().flush().unwrap();
                        },
                    }
                }
            }
        }
    }

}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long, short, default_value = ".")]
    repo_path: String,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        /// Where to create the repository.
        #[arg(default_value = ".")]
        path: String,
    },
    #[command(subcommand_value_name = "cat-file")]
    CatFile {
        #[arg(value_enum, value_name = "TYPE")]
        obj_type: GitObjectType,
        #[arg(value_name = "OBJECT")]
        object: String,
    },
}