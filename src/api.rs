//! API client implementation for the VintageStory Web Mod API.
use std::fmt::{Display, Formatter};
use crate::{error::ApiError, models::*};
use reqwest::Client;
use std::sync::Mutex;

/// The core API client for interacting with the VintageStory mod database.
#[derive(Debug)]
pub struct VintageStoryModDbApi {
    client: Client,
    enable_cache: bool,

    mods_cache: Mutex<Option<Vec<SimpleMod>>>,
    authors_cache: Mutex<Option<Vec<Author>>>,
}

impl VintageStoryModDbApi {
    const BASE_URL: &'static str = "https://mods.vintagestory.at/api";

    /// Create a new API client instance.
    ///
    /// If `enable_cache` is true, results from `/mods` and `/authors` will be cached in memory. Recommended if you will be making several calls with the same client
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
        self.get_mod_from_alias(mod_id.to_string()).await
    }

    /// Get detailed mod information from a mod alias
    pub async fn get_mod_from_alias(&self, alias: impl AsRef<str>) -> Result<DetailedMod, ApiError> {
        let url = format!("{}/mod/{}", Self::BASE_URL, alias.as_ref());
        let resp = self.client.get(url).send().await?;
        let mod_response: ModResponse = resp.json().await?;
        Ok(mod_response.mod_info)
    }

    /// Convert a simple mod to detailed mod information (helper convenience method).
    pub async fn get_detailed_mod_from_simple(&self, simple: SimpleMod) -> Result<DetailedMod, ApiError> {
        self.get_mod(simple.mod_id).await
    }

    /// Search for mods using the site. Set ascending to true to search in ascending order, and set sort_by to None to search in recently updated
    pub async fn search_mods(&self, query: impl AsRef<str>, ascending: bool, sort_by: impl Into<Option<SortBy>>) -> Result<Vec<SimpleMod>, ApiError> {
        let sort_by = sort_by.into().unwrap_or_default();

        let url = format!("{}/mods?text={}&sortby={}&sortdir={}&side=&userid=0&mv=", Self::BASE_URL, query.as_ref(), sort_by, if ascending { "a" } else { "d" });
        let resp = self.client.get(url).send().await?;

        let mods: ModsResponse = resp.json().await?;
        Ok(mods.mods)
    }

    /// Search mods by name
    pub async fn search_name(&self, query: impl AsRef<str>) -> Result<Vec<SimpleMod>, ApiError> {
        let mods = self.get_mods().await?;

        Ok(mods.into_iter().filter(|m| m.name.replace(|c| char::is_ascii_whitespace(&c) || char::is_ascii_punctuation(&c), "").eq_ignore_ascii_case(&query.as_ref().replace(|c| char::is_ascii_whitespace(&c) || char::is_ascii_punctuation(&c), ""))).collect())
    }

    /// Search mods by mod id
    pub async fn search_mod_id(&self, query: impl AsRef<str>) -> Result<Vec<SimpleMod>, ApiError> {
        let mods = self.get_mods().await?;

        Ok(mods.into_iter().filter(|m| m.mod_id_strs.contains(&query.as_ref().replace(|c| char::is_ascii_whitespace(&c) || char::is_ascii_punctuation(&c), "").to_ascii_lowercase())).collect())
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

    pub async fn get_most_recent_release(&self, mod_id: u32) -> Result<DetailedModRelease, ApiError> {
        self.get_most_recent_release_from_alias(mod_id.to_string()).await
    }

    pub async fn get_most_recent_release_from_alias(&self, alias: impl AsRef<str>) -> Result<DetailedModRelease, ApiError> {
        self.get_most_recent_release_from_alias_with_version(alias, self.get_most_recent_stable_game_version().await?.name).await
    }

    pub async fn get_most_recent_release_with_version(&self, mod_id: u32, version: impl AsRef<str>) -> Result<DetailedModRelease, ApiError> {
        self.get_most_recent_release_from_alias_with_version(mod_id.to_string(), version).await
    }

    pub async fn get_most_recent_game_version(&self) -> Result<GameVersion, ApiError> {
        let versions = self.get_game_versions().await?;

        Ok(versions.last().unwrap().clone())
    }

    pub async fn get_most_recent_stable_game_version(&self) -> Result<GameVersion, ApiError> {
        let versions = self.get_game_versions().await?;
        let mut dif = 1;
        while versions[versions.len() - dif].name.contains("pre") || versions[versions.len() - dif].name.contains("rc") || versions[versions.len() - dif].name.contains("dev") {
            dif += 1;
        }

        Ok(versions[versions.len() - dif].clone())
    }

    pub async fn get_most_recent_release_from_alias_with_version(&self, alias: impl AsRef<str>, version: impl AsRef<str>) -> Result<DetailedModRelease, ApiError> {
        let mod_info = self.get_mod_from_alias(alias).await?;
        let mut iter = mod_info.releases.iter();
        let mut current = iter.next().unwrap();
        if current.tags.contains(&version.as_ref().to_string()) {
            return Ok(current.clone());
        }

        while let Some(release) = iter.next() {
            if release.tags.contains(&version.as_ref().to_string()) {
                current = release;
                break;
            }
        }

        Ok(current.clone())
    }
}

#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum SortBy {
    Trending,
    Downloads,
    Comments,
    Name,
    #[default]
    Released,
    Created,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SortBy::Trending => "trendingpoints",
            SortBy::Downloads =>     "downloads",
            SortBy::Comments => "comments",
            SortBy::Name =>     "name",
            SortBy::Released => "lastreleased",
            SortBy::Created =>     "created",
        })
    }
}

// Optional feature: random selection functions using `rand`
#[cfg(feature = "random")]
mod random_api {
    use rand::prelude::IndexedRandom;
    use super::*;
    use rand::rng;

    impl VintageStoryModDbApi {
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
