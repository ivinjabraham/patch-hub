mod app;
mod cli;
mod handler;
mod infrastructure;
mod lore;
mod macros;
mod ui;

use app::{config::Config, App};
use clap::Parser;
use cli::Cli;
use color_eyre::eyre::bail;
use handler::run_app;
use infrastructure::{
    logging::Logger,
    terminal::{init, restore},
};
use std::ops::ControlFlow;

fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();

    infrastructure::errors::install_hooks()?;
    let mut terminal = init()?;

    let config = Config::build();
    config.create_dirs();

    match args.resolve(terminal, &config) {
        ControlFlow::Break(b) => return b,
        ControlFlow::Continue(t) => terminal = t,
    }

    let app = App::new(config)?;
    if !app.check_external_deps() {
        Logger::error("patch-hub cannot be executed because some dependencies are missing");
        bail!("patch-hub cannot be executed because some dependencies are missing, check logs for more information");
    }

    run_app(terminal, app)?;
    restore()?;

    Logger::info("patch-hub finished");
    Logger::flush();

    Ok(())
}
