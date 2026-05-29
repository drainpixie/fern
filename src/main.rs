mod cli;
mod commands;
mod layers;
mod out;

use anyhow::Result;
use clap::Parser;
use cli::{App, Command, Format};
use layers::Git;

fn main() -> Result<()> {
    let app = App::parse();
    let layer = Git::new();

    match app.command {
        None => commands::status::run(&layer, &Format::Text),

        Some(Command::Init) => commands::init::run(&layer),
        Some(Command::Status(a)) => commands::status::run(&layer, &a.format),
        Some(Command::Remotes(a)) => commands::remotes::run(&layer, &a.format),
        Some(Command::Add(a)) => commands::add::run(&layer, &a.name, &a.url),
        Some(Command::Remove(a)) => commands::remove::run(&layer, &a.name, a.yes),
        Some(Command::Rename(a)) => commands::rename::run(&layer, &a.old, &a.new),
        Some(Command::Push(a)) => commands::sync::run(&layer, commands::sync::Op::Push, &a.remotes),
        Some(Command::Pull(a)) => commands::sync::run(&layer, commands::sync::Op::Pull, &a.remotes),
        Some(Command::Fetch(a)) => {
            commands::sync::run(&layer, commands::sync::Op::Fetch, &a.remotes)
        }
    }
}
