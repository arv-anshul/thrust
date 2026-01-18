use chrono::{DateTime, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use tabled::Tabled;

use crate::models::repo::RepoEntity;

#[derive(Queryable, Selectable, Tabled)]
#[diesel(table_name = crate::schema::repo_releases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[tabled(rename_all = "PascalCase")]
pub struct RepoReleaseRow {
    pub id: i32,
    pub repo_id: i32,
    #[tabled(skip)]
    pub url: String,
    pub html_url: String,
    pub tag_name: String,
    #[tabled(skip)]
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Tabled)]
#[diesel(table_name = crate::schema::repo_releases)]
pub struct NewRepoRelease<'a> {
    pub repo_id: i32,
    pub url: &'a str,
    pub html_url: &'a str,
    pub tag_name: &'a str,
    pub body: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RepoReleaseAPI {
    pub url: String,
    pub html_url: String,
    pub tag_name: String,
    pub body: String,
    #[serde(deserialize_with = "from_github_datetime")]
    pub created_at: NaiveDateTime,
}

impl RepoReleaseAPI {
    /// Fetch releases from GitHub API
    pub fn fetch_repo_releases(
        repo: &RepoEntity,
        last_n: u8,
    ) -> Result<Vec<RepoReleaseAPI>, String> {
        let response = ureq::get(format!("https://api.github.com/repos/{}/releases", repo))
            .query("per_page", last_n.to_string())
            .header("accept", "application/vnd.github+json")
            .call()
            .expect("Error while sending requesting repo releases.")
            .body_mut()
            .read_json::<Vec<RepoReleaseAPI>>()
            .expect("Error deserializing response body.");
        Ok(response)
    }

    /// Converts the API response into a database insertable struct
    #[allow(clippy::wrong_self_convention)]
    pub fn to_insterable<'a>(&'a self, repo_id: i32) -> NewRepoRelease<'a> {
        NewRepoRelease {
            repo_id,
            url: &self.url,
            html_url: &self.html_url,
            tag_name: &self.tag_name,
            body: &self.body,
            created_at: self.created_at,
        }
    }
}

/// Custom `Datetime<Utc>` parser for `serde` to comply with `diesel` crate.
fn from_github_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?
        .naive_utc())
}
