//! VintageStory Web Mod API Client Library
//!
//! This library provides a Rust API client for interacting with the VintageStory mod database
//! at [https://mods.vintagestory.at](https://mods.vintagestory.at).  
//!
//! Features:
//! - Fetch mods, detailed mod info, authors, tags, game versions, comments
//! - Optional in-memory caching
//! - Optional random selection (via `rand` feature)

pub mod api;
pub mod error;
pub mod models;

pub use api::VintageStoryModDbApi;
pub use error::ApiError;
pub use models::*;
