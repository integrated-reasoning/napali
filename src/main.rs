//#![warn(clippy::pedantic)]
//#![warn(missing_docs)]

mod action;
mod app;
mod cli;
mod components;
mod config;
mod irx_client;
mod router;
mod tui;
mod utils;

use crate::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler},
};
use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

async fn tokio_main() -> Result<()> {
  let args = Cli::parse();
  if args.console_subscriber {
    console_subscriber::init();
  } else {
    initialize_logging()?;
  }
  initialize_panic_handler()?;
  let mut app = Box::pin(App::new(args.tick_rate, args.frame_rate)).await?;
  app.run().await?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  if let Err(e) = tokio_main().await {
    eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
    Err(e)
  } else {
    Ok(())
  }
}
