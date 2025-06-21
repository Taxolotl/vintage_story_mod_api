# Vintage Story Mod API (Rust Client Library)

[![Crates.io](https://img.shields.io/crates/v/vintage_story_mod_api.svg)](https://crates.io/crates/vintage_story_mod_api)

A fully asynchronous Rust client for accessing the Vintage Story Web Mod API (`mods.vintagestory.at`).

Supports:

- Fetching all mods (basic or detailed)
- Getting tags, authors, game versions, and comments
- Optional caching of mods & authors
- Random mod/tag/author/game-version retrieval (optional feature)

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
vintage_story_mod_api = "0.1"
```

or `cargo add vintage_story_mod_api`