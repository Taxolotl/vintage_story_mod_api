use vintage_story_mod_api::VintageStoryModApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = VintageStoryModApi::new(false);

    // Get mods to pick one asset ID
    let mods = api.get_mods().await?;
    let first_mod = mods.first().expect("No mods found");

    println!("Getting comments for mod: {} (asset_id: {})", first_mod.name, first_mod.asset_id);
    let comments = api.get_comments(first_mod.asset_id).await?;

    if comments.is_empty() {
        println!("No comments found.");
    } else {
        println!("Found {} comments:", comments.len());
        for comment in &comments {
            println!("- {} (posted at: {})", comment.text, comment.created);
        }
    }

    Ok(())
}
