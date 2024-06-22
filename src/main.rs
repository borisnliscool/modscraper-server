use std::{env, fs};
use std::io::Read;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::settings::PaginationSetting;
use tokio::net::TcpListener;

use crate::models::GameMod;

mod models;
mod server;

lazy_static! {
    static ref CLIENT: Client = Client::new(
        env::var("MEILI_HOST").expect("MEILI_HOST must be set"),
        Some(env::var("MEILI_MASTER_KEY").expect("MEILI_MASTER_KEY must be set"))
    ).unwrap();
}

fn get_mods() -> Vec<GameMod> {
    let mut file = fs::File::open("assets/mods.json").expect("Failed to open assets/mods.json");

    let mut mods_json = String::new();
    file.read_to_string(&mut mods_json).expect("Failed to read assets/mods.json");

    serde_json::from_str(&mods_json).unwrap()
}

async fn setup() -> Result<Index, String> {
    println!("Attempting to setup...");

    let mods_list = get_mods();

    match CLIENT.create_index("mods", None).await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create index: {}", e))
    }

    let mods_index = CLIENT.index("mods");

    match mods_index.add_documents(&mods_list, Some("id")).await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to add documents: {}", e))
    }

    match mods_index.set_ranking_rules([
        "words",
        "downloads:desc",
        "typo",
        "proximity",
        "attribute",
        "exactness",
        "sort",
    ]).await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to set ranking rules: {}", e))
    }

    match mods_index.set_pagination(PaginationSetting { max_total_hits: 1_000_000 }).await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to set pagination: {}", e))
    }

    match mods_index.set_filterable_attributes(["id", "categories", "author"]).await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to set filterable attributes: {}", e))
    }

    println!("Indexed {} documents", mods_list.len());

    Ok(mods_index)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    match setup().await {
        Ok(_) => println!("Setup complete"),
        Err(e) => panic!("Failed to setup: {}", e)
    };

    let address = format!(
        "{}:{}",
        env::var("MODSCRAPER_PORT").unwrap_or(3000.to_string()),
        env::var("MODSCRAPER_HOST").unwrap_or("server".to_string())
    );

    println!("Starting server on {}", address);

    let listener = TcpListener::bind(address.trim()).await.expect("Failed to bind to port 8080");

    let router = server::create();
    axum::serve(listener, router).await.expect("Failed to start server");
}
