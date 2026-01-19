use crate::models::repo::RepoEntity;
use crate::sql::repo_release::RepoReleaseRow;
use crate::utils::markdown::display_markdown;
use diesel::prelude::*;

pub fn show_releases(conn: &mut SqliteConnection, repo_entity: &RepoEntity) {
    let repo = repo_entity.get_from_db(conn).unwrap_or_else(|_| {
        eprintln!("Repository '{}' not found!", repo_entity);
        std::process::exit(1);
    });
    let release = RepoReleaseRow::get_from_repo(conn, &repo).expect("Error getting release note");

    display_markdown(release.body);
}
