use crate::models::repo::RepoEntity;
use crate::schema::*;
use crate::utils::markdown::display_markdown;
use diesel::prelude::*;

pub fn show_releases(conn: &mut SqliteConnection, repo: &RepoEntity) {
    let repo_id: i32 = repos::table
        .filter(repos::owner.eq(&repo.owner))
        .filter(repos::name.eq(&repo.name))
        .select(repos::id)
        .first(conn)
        .unwrap_or_else(|_| {
            eprintln!("Repository '{}' not found!", repo);
            std::process::exit(1);
        });
    let release_note = repo_releases::table
        .filter(repo_releases::repo_id.eq(repo_id))
        .order(repo_releases::created_at.desc())
        .select(repo_releases::body)
        .first::<String>(conn)
        .expect("Error getting release note");

    display_markdown(release_note);
}
