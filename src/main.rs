use std::error::Error;

use clap::Parser;
use notify::{RecursiveMode, Watcher};
use relm::Widget;

use langview::gui::Langview;
use langview::watch::async_watcher;

#[derive(Parser)]
pub struct Args {
    /// Sourceview Lang file to watch
    #[clap(short, long)]
    pub lang: String,

    /// Source file to render
    #[clap(short, long)]
    pub test: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let (mut watcher, rx) = async_watcher()?;
    watcher.watch(args.lang.as_ref(), RecursiveMode::NonRecursive)?;

    Langview::run((args.lang.clone(), args.test.clone(), rx)).expect("App::run failed");

    Ok(())
}
