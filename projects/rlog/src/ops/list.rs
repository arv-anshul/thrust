use crate::models::repo::RepoEntity;
use crate::schema::*;
use crate::sql::repo::RepoEntityRow;
use crate::sql::repo_release::RepoReleaseRow;
use crate::utils::table::display_table;

use diesel::prelude::*;

pub fn list_repos(conn: &mut SqliteConnection) {
    match repos::table.load::<RepoEntityRow>(conn) {
        Ok(repos_vec) => {
            if repos_vec.is_empty() {
                eprintln!("0 repository found!");
                return;
            }

            display_table(&repos_vec);
        }
        Err(e) => eprintln!("Error listing repos: {e}"),
    }
}

pub fn list_repo_releases(conn: &mut SqliteConnection, repo_entity: Option<&RepoEntity>) {
    let results: Vec<RepoReleaseRow> = match repo_entity {
        // Filter for requested repository
        Some(repo) => {
            let repo = repo.get_from_db(conn).unwrap_or_else(|_| {
                eprintln!("Repository '{}' not found!", repo);
                std::process::exit(1);
            });
            repo_releases::table
                .filter(repo_releases::repo_id.eq(repo.id))
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

    display_table(&results);
}
