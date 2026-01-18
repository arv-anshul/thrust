# `rlog`

Read release notes of GitHub repositories from terminal. Written in Rust.

Just learning Rust and concepts from this project.

## Usage

1. Set current working directory of your terminal to `rlog`:
   ```bash
   cd crates/rlog
   ```
2. Setup the database using [`diesel` CLI](https://diesel.rs/guides/getting-started#installing-diesel-cli):
   ```bash
   diesel migration run
   ```
3. Run the help command to see all the commands in `rlog` CLI:
   ```bash
   cargo run -- help
   ```

## Roadmap and Learning

- [x] `serde`: Crate to perform JSON serialization and deserialization in Rust.
- [x] `clap`: Rust crate to create CLI tool.
  - [x] `clap::Parser`, `clap::Args`, and `clap::Subcommand`
- [x] `diesel`: Type safe ORM for Rust. _(but it's only synchronous)_
  - [x] Table declaration and CRUD operations.
  - [x] Implement table relationship.
  - [x] Implement cascade delete for `repo_releases` table.
- [x] Use **Clippy** for linting.
- [ ] Introduce Rust lifetime to struct.
- [ ] Async implementation.
  - I don't think it's possible with this project, as `diesel` doesn't supports it right now. Although, I can use
    `reqwest` in place of `ureq` for async feature. I'll see.
- Refactor project.
  - [ ] `tabled`: Table creation and displaying.
  - [ ] Database tables and Serde models.
  - [ ] `crate::ops::*` crate functions.
