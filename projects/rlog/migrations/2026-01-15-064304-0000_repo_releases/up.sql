CREATE TABLE repo_releases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    repo_id INTEGER NOT NULL,
    url VARCHAR NOT NULL,
    html_url VARCHAR NOT NULL,
    tag_name VARCHAR NOT NULL,
    body TEXT NOT NULL,
    created_at DATETIME NOT NULL,

    -- Define the relationship
    FOREIGN KEY (repo_id) REFERENCES repos (id) ON DELETE CASCADE
);
