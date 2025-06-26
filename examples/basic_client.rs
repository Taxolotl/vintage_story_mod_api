use vintagestory_mod_db_api::VintageStoryModDbApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the client (without caching enabled)
    let api = VintageStoryModDbApi::new(false);

    // Get all mods
    let mods = api.get_mods().await?;
    println!("Found {} mods", mods.len());

    // Get details for the first mod
    if let Some(first_mod) = mods.first() {
        println!("First mod: {} (mod_id: {})", first_mod.name, first_mod.mod_id);

        let detailed = api.get_mod(first_mod.mod_id).await?;
        println!("Detailed mod description: {}", detailed.text);
    }

    // Get tags
    let tags = api.get_tags().await?;
    println!("Available tags:");
    for tag in tags.iter().take(5) {
        println!(" - {} (id: {})", tag.name, tag.tag_id);
    }

    // Get authors
    let authors = api.get_authors().await?;
    println!("Found {} authors", authors.len());

    Ok(())
}
