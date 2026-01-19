use diesel::prelude::*;

use crate::models::{repo::RepoEntity, repo_release::RepoReleaseAPI};
use crate::sql::repo_release::NewRepoRelease;

pub fn add_repo(conn: &mut SqliteConnection, entity: &RepoEntity) -> i32 {
    if entity.get_from_db(conn).is_ok() {
        eprintln!("Repository '{}' already exists!", entity);
        std::process::exit(1);
    }
    entity.insert_into_db(conn)
}

pub fn add_repo_releases(
    conn: &mut SqliteConnection,
    entity: &RepoEntity,
    repo_id: i32,
    last_n: u8,
) {
    let releases =
        RepoReleaseAPI::fetch_repo_releases(entity, last_n).expect("Failed to fetch releases");

    let releases = releases
        .iter()
        .map(|r| r.to_insterable(repo_id))
        .collect::<Vec<NewRepoRelease>>();

    NewRepoRelease::batch_insert_into_db(conn, &releases);
}
