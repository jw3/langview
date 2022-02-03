use clap::Parser;
use relm::Widget;

use langview::gui::Langview;
use langview::watch::async_watcher;
use std::error::Error;

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

    let (_watcher, rx) = async_watcher(&args.lang)?;

    Langview::run((args.lang.clone(), args.test.clone(), rx)).expect("App::run failed");

    Ok(())
}
