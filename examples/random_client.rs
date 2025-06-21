use vintage_story_mod_api::VintageStoryApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable "random" feature for this to work
    let api = VintageStoryApi::new(false);

    // Get random mod
    let random_mod = api.get_random_mod().await?;
    println!("Random mod: {}", random_mod.name);

    // Get random tag
    let tag = api.get_random_tag().await?;
    println!("Random tag: {}", tag.name);

    // Get random author
    let author = api.get_random_author().await?;
    println!("Random author: {:?}", author.name.unwrap_or_else(|| "<Unnamed>".to_string()));

    // Get random game version
    let version = api.get_random_game_version().await?;
    println!("Random game version: {}", version.name);

    Ok(())
}
