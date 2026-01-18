use crate::models::repo::RepoEntity;
use crate::schema::*;
use diesel::prelude::*;

pub fn remove_repo(conn: &mut SqliteConnection, entity: &RepoEntity) {
    // Get the id from table
    let repo_id = repos::table
        .filter(repos::owner.eq(&entity.owner))
        .filter(repos::name.eq(&entity.name))
        .select(repos::id)
        .first::<i32>(conn);

    match repo_id {
        Ok(repo_id) => {
            // Filter with id and delete the row
            diesel::delete(repos::table.filter(repos::id.eq(repo_id)))
                .execute(conn)
                .expect("Error deleting repo");

            println!("Removed repo: {}", entity);
        }
        Err(_) => eprintln!("Repo not found!"),
    }
}
