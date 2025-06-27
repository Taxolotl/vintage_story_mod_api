//! All models returned by the VintageStory Web Mod API.

use std::fmt;
use serde::{de, Deserialize, Deserializer};
use serde::de::Visitor;

/// Top-level response for `/mods`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct ModsResponse {
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: String,
    pub mods: Vec<SimpleMod>,
}

/// Top-level response for `/mod/{id}`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct ModResponse {
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: String,
    #[serde(rename(deserialize = "mod"))]
    pub mod_info: DetailedMod,
}

/// Top-level response for `/tags`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct TagsResponse {
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: String,
    pub tags: Vec<Tag>,
}

/// Top-level response for `/authors`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct AuthorsResponse {
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: String,
    pub authors: Vec<Author>,
}

/// Top-level response for `/gameversions`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct GameVersionsResponse {
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: String,
    #[serde(rename(deserialize = "gameversions"))]
    pub game_versions: Vec<GameVersion>,
}

/// Top-level response for `/comments/{assetid}`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct CommentsResponse {
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: String,
    pub comments: Vec<Comment>,
}

/// Simplified mod object returned by `/mods`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SimpleMod {
    #[serde(rename(deserialize = "modid"))]
    pub mod_id: u32,
    #[serde(rename(deserialize = "assetid"))]
    pub asset_id: u32,
    pub downloads: u32,
    pub follows: u32,
    #[serde(rename(deserialize = "trendingpoints"))]
    pub trending_points: u32,
    pub comments: u32,
    pub name: String,
    pub summary: Option<String>,
    #[serde(rename(deserialize = "modidstrs"))]
    pub mod_id_strs: Vec<String>,
    pub author: String,
    #[serde(rename(deserialize = "urlalias"))]
    pub url_alias: Option<String>,
    pub side: String,
    #[serde(rename(deserialize = "type"))]
    pub mod_type: String,
    pub logo: Option<String>,
    pub tags: Vec<String>,
    #[serde(rename(deserialize = "lastreleased"))]
    pub last_released: String,
}

impl From<SimpleMod> for DetailedMod {
    fn from(simple: SimpleMod) -> Self {
        DetailedMod {
            mod_id: simple.mod_id,
            asset_id: simple.asset_id,
            name: simple.name,
            text: simple.summary.unwrap_or_default(),
            author: simple.author,
            url_alias: simple.url_alias,
            logo_filename: simple.logo.clone(),
            logo_file: simple.logo.clone(),
            logo_file_db: simple.logo,
            homepage_url: None,
            source_code_url: None,
            trailer_video_url: None,
            issue_tracker_url: None,
            wiki_url: None,
            downloads: simple.downloads,
            follows: simple.follows,
            trending_points: simple.trending_points,
            comments: simple.comments,
            side: simple.side,
            mod_type: simple.mod_type,
            created: "Creation date not available".to_string(),
            last_released: simple.last_released,
            last_modified: "Modified date not available".to_string(),
            tags: simple.tags,
            releases: vec![],
            screenshots: vec![],
        }
    }
}

/// Full detailed mod returned by `/mod/{id}`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct DetailedMod {
    #[serde(rename(deserialize = "modid"))]
    pub mod_id: u32,
    #[serde(rename(deserialize = "assetid"))]
    pub asset_id: u32,
    pub name: String,
    pub text: String,
    pub author: String,
    #[serde(rename(deserialize = "urlalias"))]
    pub url_alias: Option<String>,
    #[serde(rename(deserialize = "logofilename"))]
    pub logo_filename: Option<String>,
    #[serde(rename(deserialize = "logofile"))]
    pub logo_file: Option<String>,
    #[serde(rename(deserialize = "logofiledb"))]
    pub logo_file_db: Option<String>,
    #[serde(rename(deserialize = "homepageurl"))]
    pub homepage_url: Option<String>,
    #[serde(rename(deserialize = "sourcecodeurl"))]
    pub source_code_url: Option<String>,
    #[serde(rename(deserialize = "trailervideourl"))]
    pub trailer_video_url: Option<String>,
    #[serde(rename(deserialize = "issuetrackerurl"))]
    pub issue_tracker_url: Option<String>,
    #[serde(rename(deserialize = "wikiurl"))]
    pub wiki_url: Option<String>,
    pub downloads: u32,
    pub follows: u32,
    #[serde(rename(deserialize = "trendingpoints"))]
    pub trending_points: u32,
    pub comments: u32,
    pub side: String,
    #[serde(rename(deserialize = "type"))]
    pub mod_type: String,
    pub created: String,
    #[serde(rename(deserialize = "lastreleased"))]
    pub last_released: String,
    #[serde(rename(deserialize = "lastmodified"))]
    pub last_modified: String,
    pub tags: Vec<String>,
    pub releases: Vec<DetailedModRelease>,
    pub screenshots: Vec<DetailedModScreenshot>,
}

impl From<DetailedMod> for SimpleMod {
    fn from(detail: DetailedMod) -> Self {
        SimpleMod {
            mod_id: detail.mod_id,
            asset_id: detail.asset_id,
            downloads: detail.downloads,
            follows: detail.follows,
            trending_points: detail.trending_points,
            comments: detail.comments,
            name: detail.name,
            summary: Some(detail.text),
            mod_id_strs: detail.releases.iter().filter_map(|r| r.mod_id_str.clone()).collect(),
            author: detail.author,
            url_alias: detail.url_alias,
            side: detail.side,
            mod_type: detail.mod_type,
            logo: detail.logo_file,
            tags: detail.tags,
            last_released: detail.last_released,
        }
    }
}

/// Mod release info (only in DetailedMod)
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct DetailedModRelease {
    #[serde(rename(deserialize = "releaseid"))]
    pub release_id: u32,
    #[serde(rename(deserialize = "mainfile"))]
    pub main_file: String,
    #[serde(deserialize_with = "string_or_null")]
    pub filename: Option<String>,
    #[serde(rename(deserialize = "fileid"))]
    pub file_id: Option<u32>,
    pub downloads: u32,
    pub tags: Vec<String>,
    #[serde(rename(deserialize = "modidstr"))]
    pub mod_id_str: Option<String>,
    #[serde(rename(deserialize = "modversion"))]
    pub mod_version: String,
    pub created: String,
    pub changelog: Option<String>,
}

impl DetailedModRelease {
    pub fn get_filename(&self) -> String {
        self.filename.clone().unwrap_or(self.mod_id_str.clone().map(|str| format!("{}.zip", str)).unwrap_or(self.main_file.clone()))
    }
}

fn string_or_null<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNullVisitor;

    impl<'de> Visitor<'de> for StringOrNullVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, integer, or null")
        }

        fn visit_i64<E>(self, _v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringOrNullVisitor)
}


/// Screenshot entry for a mod
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct DetailedModScreenshot {
    #[serde(rename(deserialize = "fileid"))]
    pub file_id: u32,
    #[serde(rename(deserialize = "mainfile"))]
    pub main_file: String,
    pub filename: String,
    #[serde(rename(deserialize = "thumbnailfilename"))]
    pub thumbnail_filename: String,
    pub created: String,
}

/// Tag object returned by `/tags`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Tag {
    #[serde(rename(deserialize = "tagid"))]
    pub tag_id: u32,
    pub name: String,
    pub color: String,
}

/// Author object returned by `/authors`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Author {
    #[serde(rename(deserialize = "userid"))]
    pub userid: u32,
    pub name: Option<String>,
}

/// GameVersion object returned by `/gameversions`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct GameVersion {
    #[serde(rename(deserialize = "tagid"))]
    pub tag_id: i64,
    pub name: String,
    pub color: String,
}

/// Comment object returned by `/comments/{assetid}`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Comment {
    #[serde(rename(deserialize = "commentid"))]
    pub comment_id: u32,
    #[serde(rename(deserialize = "assetid"))]
    pub asset_id: u32,
    #[serde(rename(deserialize = "userid"))]
    pub user_id: u32,
    pub text: String,
    pub created: String,
    #[serde(rename(deserialize = "lastmodified"))]
    pub last_modified: String,
}