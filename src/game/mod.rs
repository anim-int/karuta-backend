pub mod index;
pub mod state;
pub mod test;
pub mod websocket;

use serde;

use rocket::{
    http::Status,
    response::{content, status},
    serde::json::Json,
    State,
};
use rocket_okapi::{openapi, JsonSchema};

use state::{CardGuessResult, GameContinuation};

use std::sync::{Arc, RwLock};

use crate::{
    deck::{Card, DeckSource},
    Deck,
};

use index::*;

#[derive(Debug, Copy, Clone, serde::Serialize)]
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

#[derive(Debug, Clone, Copy, serde::Deserialize, Eq, PartialEq, JsonSchema, Default)]
pub struct GameConfigOverride {
    pub has_ffa: Option<bool>,
    pub duplicate_policy: Option<DuplicatePolicy>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Eq, PartialEq, JsonSchema)]
pub enum DuplicatePolicy {
    MatchCard,
    MatchMusic,
    MatchAnime,
}

#[derive(Debug, Clone, serde::Deserialize, Eq, PartialEq, JsonSchema)]
pub struct GameData {
    pub deck_names: Vec<String>,
    pub config_override: Option<GameConfigOverride>,
}

fn serialize_to_raw_json(obj: &impl serde::Serialize) -> content::RawJson<String> {
    content::RawJson(serde_json::ser::to_string(obj).unwrap())
}

#[openapi(tag = "Game")]
#[post("/game/create", format = "json", data = "<game_data>")]
pub fn create_game(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    game_data: Json<GameData>,
) -> status::Custom<content::RawText<String>> {
    let decks = game_data
        .deck_names
        .iter()
        .map(|name| {
            decks
                .iter()
                .find(|(_, deck)| &deck.name == name)
                .map(|(_, deck)| deck)
        })
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
    status::Custom(Status::Created, content::RawText(game_id))
}

#[openapi(tag = "Game")]
#[post("/game/create1v1/<d1>/<d2>")]
pub fn create_1v1_game(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    decks: &State<Arc<Vec<(DeckSource, Deck)>>>,
    d1: String,
    d2: String,
) -> status::Custom<content::RawText<String>> {
    let deck1 = decks
        .iter()
        .find(|(_, deck)| deck.name == d1)
        .map(|(_, deck)| deck);
    let deck2 = decks
        .iter()
        .find(|(_, deck)| deck.name == d2)
        .map(|(_, deck)| deck);

    if let (Some(deck1), Some(deck2)) = (deck1, deck2) {
        let game_id = game_index
            .write()
            .unwrap()
            .create_game(vec![deck1.cards.clone(), deck2.cards.clone()]);
        status::Custom(Status::Created, content::RawText(game_id))
    } else {
        status::Custom(
            Status::NotFound,
            content::RawText("Deck not found".to_string()),
        )
    }
}

#[openapi(tag = "Game")]
#[get("/game/<game_id>/play")]
pub fn play_card(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
) -> status::Custom<content::RawJson<String>> {
    if let Some(mut game) = game_index.read().unwrap().get_game(&game_id) {
        if game.has_game_ended() {
            status::Custom(Status::Gone, serialize_to_raw_json(&"Game has ended"))
        } else if let Some(playing_card) = game.get_current_card_playing() {
            status::Custom(Status::Ok, serialize_to_raw_json(playing_card))
        } else {
            game.play_card().unwrap();
            let playing_card = game.get_current_card_playing().unwrap();
            status::Custom(Status::Created, serialize_to_raw_json(playing_card))
        }
    } else {
        status::Custom(Status::NotFound, serialize_to_raw_json(&"Game not found"))
    }
}

#[openapi(tag = "Game")]
#[post("/game/<game_id>/guess", format = "json", data = "<card>")]
pub fn guess_card(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
    card: Json<Card>,
) -> status::Custom<content::RawJson<String>> {
    let game_index_reader = game_index.read().unwrap();
    let game = game_index_reader.get_game(&game_id);
    if game.is_none() {
        return status::Custom(Status::NotFound, serialize_to_raw_json(&"Game not found"));
    }
    let mut game = game.unwrap();
    if game.has_game_ended() {
        return status::Custom(Status::Gone, serialize_to_raw_json(&"Game has ended"));
    } else if game.get_current_card_playing().is_none() {
        return status::Custom(
            Status::BadRequest,
            serialize_to_raw_json(&"No card currently playing"),
        );
    };
    let guess_card_result = game.guess_card(card.0).unwrap();
    if guess_card_result != CardGuessResult::Correct(GameContinuation::End) {
        return status::Custom(Status::Ok, serialize_to_raw_json(&guess_card_result));
    };
    // Release locks
    drop(game);
    drop(game_index_reader);
    game_index.write().unwrap().remove_game(&game_id);
    status::Custom(
        Status::Ok,
        serialize_to_raw_json(&CardGuessResult::Correct(GameContinuation::End)),
    )
}

#[openapi(tag = "Game")]
#[get("/game/<game_id>/config")]
pub fn get_game_config(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
) -> status::Custom<content::RawJson<String>> {
    if let Some(game) = game_index.read().unwrap().get_game(&game_id) {
        status::Custom(Status::Created, serialize_to_raw_json(&game.get_config()))
    } else {
        status::Custom(Status::NotFound, serialize_to_raw_json(&"Game not found"))
    }
}

#[openapi(tag = "Game")]
#[get("/game/<game_id>/state")]
pub fn get_state(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
) -> status::Custom<content::RawJson<String>> {
    if let Some(game) = game_index.read().unwrap().get_game(&game_id) {
        status::Custom(
            Status::Created,
            serialize_to_raw_json(&game.get_game_continuation()),
        )
    } else {
        status::Custom(Status::NotFound, serialize_to_raw_json(&"Game not found"))
    }
}

#[openapi(tag = "Game")]
#[get("/game/<game_id>/boards")]
pub fn get_boards(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
) -> status::Custom<content::RawJson<String>> {
    if let Some(game) = game_index.read().unwrap().get_game(&game_id) {
        status::Custom(Status::Created, serialize_to_raw_json(&game.get_boards()))
    } else {
        status::Custom(Status::NotFound, serialize_to_raw_json(&"Game not found"))
    }
}

#[openapi(tag = "Game")]
#[get("/game/<game_id>/board/<player>")]
pub fn get_player_board(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
    player: usize,
) -> status::Custom<content::RawJson<String>> {
    if let Some(game) = game_index.read().unwrap().get_game(&game_id) {
        if let Some(board) = game.get_player_board(player) {
            status::Custom(Status::Created, serialize_to_raw_json(&board))
        } else {
            status::Custom(Status::NotFound, serialize_to_raw_json(&"Player not found"))
        }
    } else {
        status::Custom(Status::NotFound, serialize_to_raw_json(&"Game not found"))
    }
}

#[openapi(tag = "Game")]
#[post("/game/<game_id>/move/<player_source>/<index_source>/to/<player_dest>")]
pub fn move_card(
    game_index: &State<Arc<RwLock<GameIndex>>>,
    game_id: String,
    player_source: usize,
    index_source: usize,
    player_dest: usize,
) -> status::Custom<content::RawText<&str>> {
    if let Some(mut game) = game_index.read().unwrap().get_game(&game_id) {
        if game.has_game_ended() {
            status::Custom(Status::Gone, content::RawText("Game has ended"))
        } else {
            let boards = game.get_boards_mut();
            if player_source >= boards.len() || player_dest >= boards.len() {
                status::Custom(Status::NotFound, content::RawText("Player not found"))
            } else {
                let source_board = boards.get_mut(player_source).unwrap();
                if index_source >= source_board.len() {
                    status::Custom(Status::NotFound, content::RawText("Card not found"))
                } else {
                    let card = source_board.remove(index_source);
                    boards.get_mut(player_dest).unwrap().push(card);
                    status::Custom(Status::Ok, content::RawText("Success"))
                }
            }
        }
    } else {
        status::Custom(Status::NotFound, content::RawText("Game not found"))
    }
}
