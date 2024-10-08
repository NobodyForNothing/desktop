use std::{fs::File, path::PathBuf};

use arboard::Clipboard;
use clap::Parser;
use input::{cli::{AddArgs, Cli, Commands}, config::Config};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info, trace};
use simplelog::{CombinedLogger, SharedLogger, TermLogger, WriteLogger};
use snippets::{SnipetsFolderBuilder};

mod input;
mod snippets;

fn main() {
    let args = Cli::parse();

    setup_logging(&args);

    debug!("Cli arguments parsed:\n{:#?}", &args);

    if run(&args).is_none() {
        return;
    }
}

fn setup_logging(args: &Cli) {
    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();

    let log_path = dirs::data_dir().unwrap_or(PathBuf::from(".log")).join("snippets.log");
    if let Ok(file) = File::create(log_path) {
        loggers.push(WriteLogger::new(
            log::LevelFilter::Trace,
            simplelog::Config::default(),
            file,
        ));
    }
    if args.debug {
        loggers.push(TermLogger::new(
        log::LevelFilter::Trace,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::AlwaysAnsi,
        ));
    }
    // TODO: consider adding sys logging
    
    if let Err(_) = CombinedLogger::init(loggers) {
        eprintln!("Unable to setup logging");
    }
}

fn run(args: &Cli) -> Option<()> {
    // setup
    let pb = ProgressBar::new(5)
        .with_style(ProgressStyle::with_template("[{elapsed_precise}] {human_pos}/{human_len} {msg}").unwrap())
        .with_message("parsing config");

    let config = Config::load(&args)?;
    pb.inc(1);

    let pb = pb.with_message("loading snippets");
    let snippets = SnipetsFolderBuilder::new(&config);
    pb.inc(1);
    
    let pb = pb.with_message("validating snippets dir");
    let snippets = snippets.validate()?;
    pb.inc(1);
    
    let pb = pb.with_message("indexing snippets dir");
    let snippets = snippets.index()?;
    pb.inc(1);
    
    let pb = pb.with_message("opening snippets dir");
    let snippets = snippets.open();

    pb.finish();


    // arg
    match &args.command {
        None => todo!("TUI not yet implemented"),
        Some(Commands::List) => snippets.list(),
        Some(Commands::Add(AddArgs { title, tags, clipboard, lang })) => {
            let content = if clipboard.clone() {
                trace!("Loading snippet content from clipboard.");
                let mut clipboard = Clipboard::new().inspect_err(|err| {
                    error!("Unable to get clipboard.");
                    debug!("{:?}", err);
                }).ok()?;
                let txt = clipboard.get_text().inspect_err(|err| {
                    error!("Unable to read from clipboard.");
                    debug!("{:?}", err);
                }).ok()?;
                Some(txt)
            } else {
                // open input
                todo!()
            };
            if content.clone().is_some_and(|content| !content.is_empty()) {
                let content = content.expect("checked above");
                trace!("Successfully obtained clipboard content.");
                trace!("{content}");
                let tags = tags.into_iter().map(|e| e.clone()).collect::<Vec<String>>();
                snippets.add(title.clone(), tags, content);
            } else {
                error!("No content to store provided.");
                return None;
            }
        },
    }


    Some(())
}