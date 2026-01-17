// @generated automatically by Diesel CLI.

diesel::table! {
    repo_releases (id) {
        id -> Integer,
        repo_id -> Integer,
        url -> Text,
        html_url -> Text,
        tag_name -> Text,
        body -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    repos (id) {
        id -> Integer,
        owner -> Text,
        name -> Text,
    }
}

diesel::joinable!(repo_releases -> repos (repo_id));

diesel::allow_tables_to_appear_in_same_query!(repo_releases, repos,);
