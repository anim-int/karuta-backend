use schemars::JsonSchema;
use serde;

use rocket::{response::Redirect, serde::json::Json, State};
use rocket_okapi::openapi;
use std::sync::Arc;

use crate::git_repos::GitSource;

// Type alias so I don't have to rewrite some code
pub type DeckSource = GitSource;

/// Some fancy functions tu generate URLs to commonly fetched files
impl DeckSource {
    pub fn get_deck_json_url(&self) -> String {
        self.get_file_url("deck.json")
    }

    pub fn get_visual_url(&self, image_name: &str) -> String {
        self.get_file_url(format!("Visuals/{}", image_name))
    }

    pub fn get_sound_url(&self, image_name: &str) -> String {
        self.get_file_url(format!("Sounds/{}", image_name))
    }

    pub fn get_deck_cover_url(&self) -> String {
        self.get_file_url("cover.png")
    }
}

/// This structure contains all the data of a single Karuta deck, it can be serialized
/// and deserialized for transfer.
/// 
/// A `Deck` is comprised of some metadata and a set of 30 cards stored in a `Vec`.
/// 
/// The `category` and `type` fields reference the fields of [`CategoryJSON`](crate::categories::CategoryJSON)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Deck {
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub author: String,
    pub cards: Vec<Card>,
}

/// This structure contains all the data of a single Karuta card, it can be serialized
/// and deserialized for transfer.
/// 
/// Each card has a unique `id` within a deck, it is used as an identifier for fetching ressource files
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Card {
    pub id: u32,
    pub anime: String,
    pub numbering: String,
    pub title: String,
    pub authors: String,
    pub image: String,
    pub audio: String,
    pub anilist_id: u32,
}

/// Fetches the data of a deck given its name and returns it in json format
#[openapi(tag = "Decks")]
#[get("/deck/<deck_name>/metadata")]
pub async fn deck_metadata(
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    deck_name: &str,
) -> Option<Json<Deck>> {
    decks
        .iter()
        .find(|(_, deck)| deck.name == deck_name)
        .map(|(_, deck)| Json(deck.clone()))
}

/// Returns a simple new line-separated list of all the loaded decks
#[openapi(tag = "Decks")]
#[get("/deck/names")]
pub fn deck_names(decks: &State<Arc<Vec<(DeckSource, Deck)>>>) -> String {
    decks
        .iter()
        .map(|(_, deck)| deck.name.clone() + "\n")
        .collect()
}

/// Returns a redirection to the visual file of a card given its deck and id
#[openapi(tag = "Decks")]
#[get("/deck/<deck_name>/visual/<id>")]
pub async fn get_visual(
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    deck_name: &str,
    id: u32,
) -> Option<Redirect> {
    let (source, deck) = decks.iter().find(|(_, deck)| deck.name == deck_name)?;
    let card = deck.cards.iter().find(|card| card.id == id)?;
    Some(Redirect::found(source.get_visual_url(&card.image)))
}


/// Returns a redirection to the sound file of a card given its deck and id
#[openapi(tag = "Decks")]
#[get("/deck/<deck_name>/sound/<id>")]
pub async fn get_sound(
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    deck_name: &str,
    id: u32,
) -> Option<Redirect> {
    let (source, deck) = decks.iter().find(|(_, deck)| deck.name == deck_name)?;
    let card = deck.cards.iter().find(|card| card.id == id)?;
    Some(Redirect::found(source.get_sound_url(&card.audio)))
}

/// Returns a redirection to the cover file of a deck given its name
#[openapi(tag = "Decks")]
#[get("/deck/<deck_name>/cover")]
pub async fn get_cover(
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    deck_name: &str,
) -> Option<Redirect> {
    let (source, _) = decks.iter().find(|(_, deck)| deck.name == deck_name)?;
    Some(Redirect::found(source.get_deck_cover_url()))
}
