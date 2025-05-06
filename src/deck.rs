use schemars::JsonSchema;
use serde;

use rocket::{response::Redirect, serde::json::Json, State};
use rocket_okapi::openapi;
use std::sync::Arc;

use crate::git_repos::GitSource;

pub type DeckSource = GitSource;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Deck {
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub author: String,
    pub cards: Vec<Card>,
}

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

#[openapi(tag = "Decks")]
#[get("/deck/names")]
pub fn deck_names(decks: &State<Arc<Vec<(DeckSource, Deck)>>>) -> String {
    decks
        .iter()
        .map(|(_, deck)| deck.name.clone() + "\n")
        .collect()
}

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

#[openapi(tag = "Decks")]
#[get("/deck/<deck_name>/cover")]
pub async fn get_cover(
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    deck_name: &str,
) -> Option<Redirect> {
    let (source, _) = decks.iter().find(|(_, deck)| deck.name == deck_name)?;
    Some(Redirect::found(source.get_deck_cover_url()))
}
