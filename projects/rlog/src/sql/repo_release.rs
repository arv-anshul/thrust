use chrono::NaiveDateTime;
use diesel::prelude::*;
use tabled::Tabled;

use crate::{schema::repo_releases, sql::repo::RepoEntityRow};

#[derive(Queryable, Selectable, Tabled)]
#[diesel(table_name = crate::schema::repo_releases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[tabled(rename_all = "PascalCase")]
pub struct RepoReleaseRow {
    pub id: i32,
    pub repo_id: i32,
    #[tabled(skip)]
    #[allow(unused)]
    pub url: String,
    pub html_url: String,
    pub tag_name: String,
    #[tabled(skip)]
    pub body: String,
    pub created_at: NaiveDateTime,
}

impl RepoReleaseRow {
    /// Get most recent repo release from a repo id
    pub fn get_from_repo(
        conn: &mut SqliteConnection,
        repo_entity: &RepoEntityRow,
    ) -> Result<Self, diesel::result::Error> {
        repo_releases::table
            .filter(repo_releases::repo_id.eq(repo_entity.id))
            .order(repo_releases::created_at.desc())
            .first::<Self>(conn)
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = crate::schema::repo_releases)]
pub struct NewRepoRelease<'a> {
    pub repo_id: i32,
    pub url: &'a str,
    pub html_url: &'a str,
    pub tag_name: &'a str,
    pub body: &'a str,
    pub created_at: NaiveDateTime,
}

impl<'a> NewRepoRelease<'a> {
    /// Insert into database
    pub fn insert_into_db(&self, conn: &mut SqliteConnection) -> i32 {
        diesel::insert_into(repo_releases::table)
            .values(self)
            .returning(repo_releases::id)
            .get_result::<i32>(conn)
            .expect("Not able to insert repo releases")
    }

    /// Batch insert into database
    pub fn batch_insert_into_db(conn: &mut SqliteConnection, releases: &[Self]) {
        diesel::insert_into(repo_releases::table)
            .values(releases)
            .execute(conn)
            .expect("Not able to insert repo releases");
    }
}
