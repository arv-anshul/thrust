use std::str::FromStr;
use std::{fmt, str};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Queryable, Selectable, Tabled)]
#[diesel(table_name = crate::schema::repos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[tabled(rename_all = "PascalCase")]
pub struct RepoEntityRow {
    pub id: i32,
    pub owner: String,
    pub name: String,
}

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
