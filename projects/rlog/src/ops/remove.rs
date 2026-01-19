use crate::models::repo::RepoEntity;
use diesel::prelude::*;

pub fn remove_repo(conn: &mut SqliteConnection, entity: &RepoEntity) {
    match entity.get_from_db(conn) {
        Ok(repo) => {
            repo.delete_from_db(conn).expect("Error deleting repo");
            println!("Removed repo: {}", entity);
        }
        Err(_) => eprintln!("Repo not found!"),
    }
}
