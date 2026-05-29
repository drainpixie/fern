use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "fern", about, version, propagate_version = true)]
pub struct App {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialise a repository with Git and Fern
    Init,

    /// Show the current status of all tracked remotes
    Status(StatusArgs),

    /// Lists all tracked remotes
    Remotes(RemotesArgs),

    /// Add a remote to be tracked
    Add(AddArgs),

    /// Remove a remote from being tracked
    Remove(RemoveArgs),

    /// Rename a tracked remote
    Rename(RenameArgs),

    /// Push to one or more remotes
    Push(RemoteListArgs),

    /// Pull from one or more remotes
    Pull(RemoteListArgs),

    /// Fetch from one or more remotes
    Fetch(RemoteListArgs),
}

#[derive(Debug, Args)]
pub struct StatusArgs {
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

#[derive(Debug, Args)]
pub struct RemotesArgs {
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[arg(short = 'y', long = "yes", help = "Skip confirmation prompt")]
    pub yes: bool,
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RenameArgs {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Args)]
pub struct RemoteListArgs {
    pub remotes: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    Text,
    Json,
    Plain,
}
