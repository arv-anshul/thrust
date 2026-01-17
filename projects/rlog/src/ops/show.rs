use crate::models::repo::RepoEntity;
use crate::schema::*;
use diesel::prelude::*;
use termimad::crossterm::style::Color;
use termimad::*;

fn print_release_note(body: &str) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Color::Magenta);

    skin.print_text(body);
}

pub fn show_releases(conn: &mut SqliteConnection, repo: &RepoEntity) {
    let repo_id: i32 = repos::table
        .filter(repos::owner.eq(&repo.owner))
        .filter(repos::name.eq(&repo.name))
        .select(repos::id)
        .first(conn)
        .unwrap_or_else(|_| {
            eprintln!("Repository {:?} not found!", repo);
            std::process::exit(1);
        });
    let release_note = repo_releases::table
        .filter(repo_releases::repo_id.eq(repo_id))
        .order(repo_releases::created_at.desc())
        .select(repo_releases::body)
        .first::<String>(conn)
        .expect("Error getting release note");

    print_release_note(&release_note);
}
