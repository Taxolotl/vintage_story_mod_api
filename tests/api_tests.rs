use vintage_story_mod_api::{VintageStoryApi, ApiError};

#[tokio::test]
async fn test_get_mods() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let mods = api.get_mods().await?;
    assert!(!mods.is_empty(), "Mods list should not be empty");
    Ok(())
}

#[tokio::test]
async fn test_get_single_mod() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let mods = api.get_mods().await?;
    let first_mod = mods.first().expect("Expected at least one mod");
    let detailed = api.get_mod(first_mod.mod_id).await?;
    assert_eq!(detailed.mod_id, first_mod.mod_id);
    Ok(())
}

#[tokio::test]
async fn test_get_tags() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let tags = api.get_tags().await?;
    assert!(!tags.is_empty(), "Tags list should not be empty");
    Ok(())
}

#[tokio::test]
async fn test_get_authors() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let authors = api.get_authors().await?;
    assert!(!authors.is_empty(), "Authors list should not be empty");
    Ok(())
}

#[tokio::test]
async fn test_get_game_versions() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let versions = api.get_game_versions().await?;
    assert!(!versions.is_empty(), "Game versions list should not be empty");
    Ok(())
}

#[tokio::test]
async fn test_get_comments() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let mods = api.get_mods().await?;
    let first_mod = mods.first().expect("Expected at least one mod");
    let comments = api.get_comments(first_mod.asset_id).await?;
    // It's OK if no comments exist, we're just testing the endpoint works
    assert!(comments.len() >= 0);
    Ok(())
}

#[cfg(feature = "random")]
#[tokio::test]
async fn test_random_mod() -> Result<(), ApiError> {
    let api = VintageStoryApi::new(false);
    let detailed = api.get_random_mod().await?;
    assert!(!detailed.name.is_empty());
    Ok(())
}
