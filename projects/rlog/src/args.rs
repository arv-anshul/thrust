use clap::{Args, Parser, Subcommand};

use crate::models::repo::RepoEntity;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RLogParser {
    #[clap(subcommand)]
    pub command: RLogCommand,
}

#[derive(Debug, Subcommand)]
pub enum RLogCommand {
    /// Add new repo
    Add(AddArgs),

    /// Remove stored repo data
    #[command(visible_alias = "rm")]
    Remove {
        /// Repository name
        repo: RepoEntity,
    },

    /// List stored repositories
    #[command(visible_alias = "ls")]
    List {
        #[clap(subcommand)]
        command: ListCommand,
    },

    /// Show specified repo release log
    Show(ShowArgs),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Repo name
    pub repo: RepoEntity,

    /// Only list last N number of releases
    #[arg(long, default_value_t = 2)]
    pub last_n: u8,
}

#[derive(Debug, Subcommand)]
pub enum ListCommand {
    /// List all stored repositories
    Repo,

    /// List all repo releases
    Releases {
        /// Specify repository
        repo: Option<RepoEntity>,
    },
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    /// Repo name
    pub repo: RepoEntity,
}
