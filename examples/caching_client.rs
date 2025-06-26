use vintagestory_mod_db_api::VintageStoryModDbApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the client with caching enabled
    let api = VintageStoryModDbApi::new(true);

    println!("Fetching mods...");
    let mods = api.get_mods().await?;
    println!("Got {} mods", mods.len());

    println!("Fetching again to use cache...");
    let cached_mods = api.get_mods().await?;
    println!("Got {} mods from cache", cached_mods.len());

    // Refresh cache manually
    println!("Refreshing mods cache...");
    api.refresh_mods_cache().await?;
    println!("Cache refreshed.");

    Ok(())
}
