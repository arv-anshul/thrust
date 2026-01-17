use diesel::prelude::*;

use crate::models::repo_release::NewRepoRelease;
use crate::models::{repo::RepoEntity, repo_release::RepoReleaseAPI};
use crate::schema::*;

pub fn add_repo(conn: &mut SqliteConnection, entity: &RepoEntity) -> i32 {
    diesel::insert_into(repos::table)
        .values(entity)
        .returning(repos::id)
        .get_result::<i32>(conn)
        .unwrap_or_else(|_| panic!("Error adding repository {:?}", entity))
}

pub fn add_repo_releases(
    conn: &mut SqliteConnection,
    entity: &RepoEntity,
    repo_id: i32,
    last_n: u8,
) {
    let releases = RepoReleaseAPI::fetch_repo_releases(entity, last_n)
        .unwrap()
        .into_iter()
        .map(|r| r.to_insterable(repo_id))
        .collect::<Vec<NewRepoRelease>>();

    diesel::insert_into(repo_releases::table)
        .values(releases)
        .execute(conn)
        .expect("Not able to insert repo releases");
}
