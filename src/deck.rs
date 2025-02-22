use serde;

use rocket::{fs::NamedFile, State};
use rocket_okapi::openapi;
use std::{path::Path, sync::Arc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Deck {
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub cover: String,
    pub cards: Vec<Card>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub anime: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub visual: String,
    pub audio: String,
}

#[openapi(tag = "Decks")]
#[get("/deck/metadata/<name>")]
pub async fn deck_metadata(decks: &State<Arc<Vec<Deck>>>, name: &str) -> Option<String> {
    serde_json::to_string(decks.iter().find(|deck| deck.name == name)?).ok()
}

#[openapi(tag = "Decks")]
#[get("/deck/names")]
pub fn deck_names(decks: &State<Arc<Vec<Deck>>>) -> String {
    decks.iter().map(|deck| deck.name.clone() + "\n").collect()
}

#[openapi(tag = "Decks")]
#[get("/visual/<name>")]
pub async fn get_visual(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Visuals/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Decks")]
#[get("/sound/<name>")]
pub async fn get_sound(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Sounds/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Decks")]
#[get("/deck/cover/<name>")]
pub async fn get_cover(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Covers/{name}")))
        .await
        .ok()
}
