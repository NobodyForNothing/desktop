use clap::{command, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom data dir.
    /// 
    /// Defaults to `$XDG_DOCUMENTS_DIR/snippets`.
    #[arg(short, long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Log debugging information directly to stdout.
    #[arg(long)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a named snipped.
    Add(AddArgs),
    /// List all available snippeds
    List,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Short display title and internal file name.
    pub title: String,

    /// List of tags to find notes more quickly.
    pub tags: Vec<String>,

    /// Automatically create note from clipboard.
    #[arg(short, long)]
    pub clipboard: bool,

    /// Programming language for more automatic snippet insisghts.
    #[arg(short, long)]
    pub lang: Option<String>,
}