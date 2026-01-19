use std::str::FromStr;
use std::{fmt, str};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::repos;
use crate::sql::repo::RepoEntityRow;

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::repos)]
pub struct RepoEntity {
    pub owner: String,
    pub name: String,
}

impl RepoEntity {
    pub fn from_url(url: &str) -> Result<RepoEntity, String> {
        let remainder = url
            .trim()
            .strip_prefix("https://github.com/")
            .ok_or_else(|| "Repo URL does not start with 'https://github.com/'".to_string())?
            .trim_end_matches('/')
            .trim_end_matches(".git");

        RepoEntity::from_str(remainder)
    }

    /// Get the entity from database
    pub fn get_from_db(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<RepoEntityRow, diesel::result::Error> {
        repos::table
            .filter(repos::owner.eq(&self.owner))
            .filter(repos::name.eq(&self.name))
            // .select(repos::id)
            .first::<RepoEntityRow>(conn)
    }

    /// Insert the current entity into database
    pub fn insert_into_db(&self, conn: &mut SqliteConnection) -> i32 {
        diesel::insert_into(repos::table)
            .values(self)
            .returning(repos::id)
            .get_result::<i32>(conn)
            .unwrap_or_else(|_| panic!("Error adding repository '{}'", self))
    }
}

impl str::FromStr for RepoEntity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Checks if string is URL
        if s.starts_with("https://") {
            return Self::from_url(s);
        }

        // Checks if string contains only one '/'
        if s.find('/') != s.rfind('/') {
            return Err(format!("Invalid format: '{}'. expected 'owner/name'", s));
        }

        // We expect the format "owner/name"
        let (owner, name) = s
            .split_once('/')
            .ok_or_else(|| format!("Invalid format: '{}'. expected 'owner/name'", s))?;

        // specific validation logic
        if owner.trim().is_empty() || name.trim().is_empty() {
            return Err("owner or name cannot be empty".to_string());
        }

        Ok(RepoEntity {
            owner: owner.to_string(),
            name: name.to_string(),
        })
    }
}

impl fmt::Display for RepoEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl From<RepoEntity> for String {
    fn from(val: RepoEntity) -> Self {
        val.to_string()
    }
}

impl TryFrom<String> for RepoEntity {
    type Error = <RepoEntity as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        RepoEntity::from_str(&value)
    }
}
