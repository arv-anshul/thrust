use crate::models::{repo::RepoEntity, repo::RepoEntityRow, repo_release::RepoReleaseRow};
use crate::schema::*;

use diesel::prelude::*;
use tabled::{
    Table,
    settings::{
        Alignment, Color, Style,
        object::{Columns, Rows},
    },
};

pub fn list_repos(conn: &mut SqliteConnection) {
    match repos::table.load::<RepoEntityRow>(conn) {
        Ok(repos_vec) => {
            if repos_vec.is_empty() {
                eprintln!("0 repository found!");
                return;
            }

            let mut table = Table::new(repos_vec);
            table.with(Style::modern_rounded());
            table.modify(Columns::first(), Alignment::right());
            table.modify(Columns::first(), Color::rgb_fg(255, 200, 0));
            table.modify(Rows::first(), Color::rgb_fg(255, 0, 0));

            println!("{}", table);
        }
        Err(e) => eprintln!("Error listing repos: {e}"),
    }
}

pub fn list_repo_releases(conn: &mut SqliteConnection, repo_entity: Option<&RepoEntity>) {
    let results: Vec<RepoReleaseRow> = match repo_entity {
        // Filter for requested repository
        Some(repo) => {
            let repo_id: i32 = repos::table
                .filter(repos::owner.eq(&repo.owner))
                .filter(repos::name.eq(&repo.name))
                .select(repos::id)
                .first(conn)
                .unwrap_or_else(|_| {
                    eprintln!("Repository '{}' not found!", repo);
                    std::process::exit(1);
                });
            repo_releases::table
                .filter(repo_releases::repo_id.eq(repo_id))
                .load(conn)
                .expect("Error loading Repo releases")
        }
        // No filtering
        None => repo_releases::table
            .load(conn)
            .expect("Error loading Repo releases"),
    };

    if results.is_empty() {
        match repo_entity {
            Some(repo) => eprintln!("Repository '{}' not exists!", repo),
            None => eprintln!("0 releases found!"),
        }
        std::process::exit(1);
    }

    let mut table = Table::new(results);
    table.with(Style::modern_rounded());
    table.modify(Columns::first(), Alignment::right());
    table.modify(Columns::first(), Color::rgb_fg(255, 200, 0));
    table.modify(Rows::first(), Color::rgb_fg(255, 0, 0));

    println!("{}", table);
}
