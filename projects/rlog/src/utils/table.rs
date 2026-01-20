use tabled::{
    Table, Tabled,
    settings::{Color, Style, object::Rows},
};

/// Display table from `records` using `tabled::Table`.
pub fn create_table<T: Tabled>(records: &[T]) -> Table {
    let mut table = Table::new(records);
    table
        .with(Style::modern_rounded())
        .modify(Rows::first(), Color::FG_BRIGHT_BLUE);

    table
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use tabled::assert::assert_width;

    use super::create_table;
    use crate::sql::{repo::RepoEntityRow, repo_release::RepoReleaseRow};

    #[test]
    fn test_repo_table_width() {
        let records = vec![
            RepoEntityRow {
                id: 1,
                owner: "arv-anshul".into(),
                name: "thrust".into(),
            },
            RepoEntityRow {
                id: 3,
                owner: "tracel-ai".into(),
                name: "burn".into(),
            },
        ];
        let table = create_table(&records);

        assert_width!(table, 58);
    }

    #[test]
    fn test_repo_releases_table_width() {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let records = vec![
            RepoReleaseRow {
                id: 1,
                repo_id: 3,
                url: "https://github.com/arv-anshul/thrust".into(),
                html_url: "https://github.com/arv-anshul/thrust/dummy".into(),
                tag_name: "v0.1.0".into(),
                body: "This is dummy release note!".into(),
                created_at: NaiveDateTime::parse_from_str("2026-01-01 01:02:03", fmt).unwrap(),
            },
            RepoReleaseRow {
                id: 1,
                repo_id: 3,
                url: "https://github.com/tracel-ai/burn".into(),
                html_url: "https://github.com/tracel-ai/burn/dummy".into(),
                tag_name: "v0.1.0".into(),
                body: "This is dummy release note for tracel-ai/burn repo!".into(),
                created_at: NaiveDateTime::parse_from_str("2026-01-01 01:02:03", fmt).unwrap(),
            },
        ];
        let table = create_table(&records);

        assert_width!(table, 142);
    }
}
