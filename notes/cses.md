# cses — Progress Tracker

## Purpose

`cses.rs` is the default binary (`cargo run`). It tracks progress across all 400 CSES problems and provides three views of the solution catalog.

## Usage

| Command                              | Output                                                |
|--------------------------------------|-------------------------------------------------------|
| `cargo run`                          | Summary: usage instructions + X/400 count             |
| `cargo run -- --list`                | Flat column list of completed solutions               |
| `cargo run -- --categories`          | Completed solutions grouped by CSES category          |
| `cargo run -- --all`                 | All 400 problems in columns; green = done, red = todo |

## Data Source

Problem metadata lives in `problems.json` at the workspace root — 18 CSES categories, 400 problems, each with:

- `id` — CSES task ID (used to construct `https://cses.fi/problemset/task/{id}`)
- `name` — display name as shown on CSES (e.g. `"Weird Algorithm"`)
- `slug` — filesystem-safe name (e.g. `"weird_algorithm"`)

The JSON was scraped from the CSES problem list and should be updated when new problems are added.

## Completion Detection

A problem is considered complete if `src/bin/{slug}.rs` exists. No manual tracking needed — adding a solution file is sufficient.

```rust
fn is_complete(&self) -> bool {
    Path::new(&format!("src/bin/{}.rs", self.slug)).exists()
}
```

**Important:** slugs in `problems.json` must exactly match the filenames in `src/bin/`. If a filename diverges from the CSES name (e.g. a shorter abbreviation), update the file to match the slug.

## Techniques

- **`serde` / `serde_json`** — deserialize `problems.json` into typed `Category` / `Problem` structs at startup
- **Filesystem probe** — `Path::exists()` for zero-maintenance completion tracking
- **ANSI escape codes** — `\x1b[32m` (green), `\x1b[31m` (red), `\x1b[1m` (bold) applied inline; no external crate needed
- **Dynamic column layout** — column width derived from the longest slug in each group; columns fit within a fixed 100-char terminal width using integer division
- **Iterator chaining** — `flat_map` / `filter` / `count` across the nested category → problem structure for aggregation
