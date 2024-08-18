# 🦀 The Rust

I have learned Python programming language and now I am learning Rust programming language to understand low-level
concepts like Stack, Heap, Ownership & Borrowing (specific to Rust), different data-types (`u8`-`u128`, `i8`-`i128`,
`char`, ...), etc.

During my learning I am practicing and creating some programs. For example, I've built "Stone, Paper, Scissor" game and
also written a small cooridantes system program in Rust.

Learing Rust using only free resources (available on internet) from YouTube videos, Rust Books and ChatBots (
[Phind.com](https://phind.com), [perplexity.ai](https://perplexity.ai)).

## 🗂️ Showcase

1. [`hand_game`](src/bin/hand_game.rs): The "Stone, Paper, Scissor" Game

   - Uses `Enum`, `Struct`, `Result`, `match` statements, `while` loop, functions.
   - Uses `rand` crate to generate random numbers.

   ```bash
   cargo run --bin hand_game
   ```

2. [`coordinates`](src/bin/coordinates.rs): Co-ordinate System program

   - Uses `Sruct`, `impl`, implement operators (`Add`, `Sub`).
   - Implement `fmt::Display` for `Point` struct.
   - Written methods like `euclidean_distance`, `is_collinear`, etc.
   - This is not a game or any CLI program. Just practicing `strcut`.

   ```bash
   cargo run --bin coordinates
   ```

3. [`md_badges`](md_badges): CLI tool to access simpleicons md badges

   - Modular structure.
   - Uses `ureq` carte to fetch simpleicons data from GitHub, see [data.rs](md_badges/src/data.rs).
   - Uses `serde` for obvious reasons.
   - Uses `std::env` crate for cli args parsing.
   - See [md_badges/README.md](md_badges/README.md).

   ```bash
   cargo run -p md_badges
   ```
