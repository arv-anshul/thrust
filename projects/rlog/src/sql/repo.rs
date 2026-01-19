use diesel::prelude::*;
use tabled::Tabled;

use crate::schema::repos;

#[derive(Queryable, Selectable, Tabled)]
#[diesel(table_name = crate::schema::repos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[tabled(rename_all = "PascalCase")]
pub struct RepoEntityRow {
    pub id: i32,
    pub owner: String,
    pub name: String,
}

impl RepoEntityRow {
    /// Delete current entity from database
    ///
    /// Filters with id and delete the row.
    pub fn delete_from_db(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(repos::table.filter(repos::id.eq(&self.id))).execute(conn)
    }
}
