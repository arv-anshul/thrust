#[allow(unused, dead_code)]
mod args;
mod db;
mod models;
mod ops;
mod schema;
mod utils;

use clap::Parser;

use crate::{args::ListCommand, db::establish_connection};

fn main() {
    let _args = args::RLogParser::parse();
    let conn = &mut establish_connection();

    match _args.command {
        args::RLogCommand::Add(args) => {
            let repo_id = ops::add::add_repo(conn, &args.repo);
            ops::add::add_repo_releases(conn, &args.repo, repo_id, args.last_n);
        }
        args::RLogCommand::Remove { repo } => ops::remove::remove_repo(conn, &repo),
        args::RLogCommand::List { command } => match command {
            ListCommand::Repo => ops::list::list_repos(conn),
            ListCommand::Releases { repo } => ops::list::list_repo_releases(conn, repo.as_ref()),
        },
        args::RLogCommand::Show(command) => ops::show::show_releases(conn, &command.repo),
    }
}
