pub mod index;
pub mod state;
pub mod test;

use serde::Deserialize;

use rocket::{
    http::Status,
    response::{content, status},
    serde::json::Json,
    State,
};
use rocket_okapi::{openapi, JsonSchema};

use std::sync::{Arc, RwLock};

use crate::{deck::Card, Deck};

use index::*;
use state::*;

#[derive(Debug, Copy, Clone)]
pub struct GameConfig {
    pub has_ffa: bool,
    pub duplicate_policy: DuplicatePolicy,
}

impl GameConfig {
    fn apply_override(&mut self, config_override: GameConfigOverride) {
        if let Some(has_ffa) = config_override.has_ffa {
            self.has_ffa = has_ffa;
        }
        if let Some(duplicate_policy) = config_override.duplicate_policy {
            self.duplicate_policy = duplicate_policy;
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, JsonSchema, Default)]
pub struct GameConfigOverride {
    pub has_ffa: Option<bool>,
    pub duplicate_policy: Option<DuplicatePolicy>,
}

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, JsonSchema)]
pub enum DuplicatePolicy {
    MatchCard,
    MatchMusic,
    MatchAnime,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, JsonSchema)]
pub struct GameData {
    pub deck_names: Vec<String>,
    pub config_override: Option<GameConfigOverride>,
}

#[openapi(tag = "Game")]
#[post("/game/create", format = "json", data = "<game_data>")]
pub fn create_game(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    decks: &State<Arc<Vec<Deck>>>,
    game_data: Json<GameData>,
) -> status::Custom<content::RawText<String>> {
    let decks = game_data
        .deck_names
        .iter()
        .map(|name| decks.iter().find(|deck| &deck.name == name))
        .collect::<Vec<Option<&Deck>>>();

    for deck in decks.iter() {
        if deck.is_none() {
            return status::Custom(
                Status::NotFound,
                content::RawText("Deck not found".to_string()),
            );
        }
    }

    let config_override = game_data.config_override.unwrap_or_default();

    let game_id = game_index
        .write()
        .unwrap()
        .create_game_with_config_override(
            decks
                .iter()
                .map(|deck| deck.unwrap().cards.clone())
                .collect(),
            config_override,
        );
    status::Custom(Status::Ok, content::RawText(game_id))
}

#[openapi(tag = "Game")]
#[post("/game/create1v1/<d1>/<d2>")]
pub fn create_1v1_game(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    decks: &State<Arc<Vec<Deck>>>,
    d1: String,
    d2: String,
) -> status::Custom<content::RawText<String>> {
    let deck1 = decks.iter().find(|deck| deck.name == d1);
    let deck2 = decks.iter().find(|deck| deck.name == d2);

    if let (Some(deck1), Some(deck2)) = (deck1, deck2) {
        let game_id = game_index
            .write()
            .unwrap()
            .create_game(vec![deck1.cards.clone(), deck2.cards.clone()]);
        status::Custom(Status::Ok, content::RawText(game_id))
    } else {
        status::Custom(
            Status::NotFound,
            content::RawText("Deck not found".to_string()),
        )
    }
}
