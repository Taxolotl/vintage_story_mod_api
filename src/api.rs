//! API client implementation for the VintageStory Web Mod API.

use crate::{error::ApiError, models::*};
use reqwest::Client;
use std::sync::Mutex;

/// The core API client for interacting with the VintageStory mod database.
#[derive(Debug)]
pub struct VintageStoryApi {
    client: Client,
    enable_cache: bool,

    mods_cache: Mutex<Option<Vec<SimpleMod>>>,
    authors_cache: Mutex<Option<Vec<Author>>>,
}

impl VintageStoryApi {
    const BASE_URL: &'static str = "https://mods.vintagestory.at/api";

    /// Create a new API client instance.
    ///
    /// If `enable_cache` is true, results from `/mods` and `/authors` will be cached in memory.
    pub fn new(enable_cache: bool) -> Self {
        Self {
            client: Client::new(),
            enable_cache,
            mods_cache: Mutex::new(None),
            authors_cache: Mutex::new(None),
        }
    }

    /// Get all mods from the API.
    ///
    /// Uses cache if enabled. Returns `SimpleMod` entries with limited information.
    pub async fn get_mods(&self) -> Result<Vec<SimpleMod>, ApiError> {
        if self.enable_cache {
            if let Some(cached) = self.mods_cache.lock().unwrap().as_ref() {
                return Ok(cached.clone());
            }
        }

        let resp = self.client.get(format!("{}/mods", Self::BASE_URL)).send().await?;
        let mods_response: ModsResponse = resp.json().await?;
        let mods = mods_response.mods;

        if self.enable_cache {
            *self.mods_cache.lock().unwrap() = Some(mods.clone());
        }
        Ok(mods)
    }

    /// Refreshes the mods cache from the API.
    pub async fn refresh_mods_cache(&self) -> Result<(), ApiError> {
        let resp = self.client.get(format!("{}/mods", Self::BASE_URL)).send().await?;
        let mods_response: ModsResponse = resp.json().await?;
        *self.mods_cache.lock().unwrap() = Some(mods_response.mods);
        Ok(())
    }

    /// Get detailed mod information for a specific mod ID.
    pub async fn get_mod(&self, mod_id: u32) -> Result<DetailedMod, ApiError> {
        let url = format!("{}/mod/{}", Self::BASE_URL, mod_id);
        let resp = self.client.get(url).send().await?;
        let mod_response: ModResponse = resp.json().await?;
        Ok(mod_response.mod_info)
    }

    /// Convert a simple mod to detailed mod information (helper convenience method).
    pub async fn get_detailed_mod_from_simple(&self, simple: SimpleMod) -> Result<DetailedMod, ApiError> {
        self.get_mod(simple.mod_id).await
    }

    /// Get all tags (always live from API, no caching).
    pub async fn get_tags(&self) -> Result<Vec<Tag>, ApiError> {
        let url = format!("{}/tags", Self::BASE_URL);
        let resp = self.client.get(url).send().await?;
        let tags: TagsResponse = resp.json().await?;
        Ok(tags.tags)
    }

    /// Get all authors.
    ///
    /// Uses cache if enabled.
    pub async fn get_authors(&self) -> Result<Vec<Author>, ApiError> {
        if self.enable_cache {
            if let Some(cached) = self.authors_cache.lock().unwrap().as_ref() {
                return Ok(cached.clone());
            }
        }

        let url = format!("{}/authors", Self::BASE_URL);
        let resp = self.client.get(url).send().await?;
        let authors_response: AuthorsResponse = resp.json().await?;
        let authors = authors_response.authors;

        if self.enable_cache {
            *self.authors_cache.lock().unwrap() = Some(authors.clone());
        }
        Ok(authors)
    }

    /// Refreshes the authors cache from the API.
    pub async fn refresh_authors_cache(&self) -> Result<(), ApiError> {
        let url = format!("{}/authors", Self::BASE_URL);
        let resp = self.client.get(url).send().await?;
        let authors_response: AuthorsResponse = resp.json().await?;
        *self.authors_cache.lock().unwrap() = Some(authors_response.authors);
        Ok(())
    }

    /// Get all game versions (always live from API, no caching).
    pub async fn get_game_versions(&self) -> Result<Vec<GameVersion>, ApiError> {
        let url = format!("{}/gameversions", Self::BASE_URL);
        let resp = self.client.get(url).send().await?;
        let versions: GameVersionsResponse = resp.json().await?;
        Ok(versions.game_versions)
    }

    /// Get all comments for a specific asset ID.
    pub async fn get_comments(&self, asset_id: u32) -> Result<Vec<Comment>, ApiError> {
        let url = format!("{}/comments/{}", Self::BASE_URL, asset_id);
        let resp = self.client.get(url).send().await?;
        let comments: CommentsResponse = resp.json().await?;
        Ok(comments.comments)
    }

    /// Clear cached mods.
    pub fn clear_mods_cache(&self) {
        self.mods_cache.lock().unwrap().take();
    }

    /// Clear cached authors.
    pub fn clear_authors_cache(&self) {
        self.authors_cache.lock().unwrap().take();
    }

    /// Clear all cached data.
    pub fn clear_all_caches(&self) {
        self.clear_mods_cache();
        self.clear_authors_cache();
    }
}

// Optional feature: random selection functions using `rand`
#[cfg(feature = "random")]
mod random_api {
    use rand::prelude::IndexedRandom;
    use super::*;
    use rand::rng;

    impl VintageStoryApi {
        /// Get a random mod from the mods list.
        pub async fn get_random_mod(&self) -> Result<DetailedMod, ApiError> {
            let mods = self.get_mods().await?;
            let mut rng = rng();
            let selected = mods.choose(&mut rng).ok_or_else(|| ApiError::Unexpected("No mods found".into()))?;
            self.get_mod(selected.mod_id).await
        }

        /// Get a random tag.
        pub async fn get_random_tag(&self) -> Result<Tag, ApiError> {
            let tags = self.get_tags().await?;
            let mut rng = rng();
            let selected = tags.choose(&mut rng).ok_or_else(|| ApiError::Unexpected("No tags found".into()))?;
            Ok(selected.clone())
        }

        /// Get a random author.
        pub async fn get_random_author(&self) -> Result<Author, ApiError> {
            let authors = self.get_authors().await?;
            let mut rng = rng();
            let selected = authors.choose(&mut rng).ok_or_else(|| ApiError::Unexpected("No authors found".into()))?;
            Ok(selected.clone())
        }

        /// Get a random game version.
        pub async fn get_random_game_version(&self) -> Result<GameVersion, ApiError> {
            let versions = self.get_game_versions().await?;
            let mut rng = rng();
            let selected = versions.choose(&mut rng).ok_or_else(|| ApiError::Unexpected("No versions found".into()))?;
            Ok(selected.clone())
        }

        /// Get a random comment for an asset (or None if no comments exist).
        pub async fn get_random_comment(&self, asset_id: u32) -> Result<Option<Comment>, ApiError> {
            let comments = self.get_comments(asset_id).await?;
            if comments.is_empty() {
                return Ok(None);
            }
            let mut rng = rng();
            Ok(Some(comments.choose(&mut rng).unwrap().clone()))
        }
    }
}
