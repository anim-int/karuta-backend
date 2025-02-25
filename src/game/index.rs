use rand::distr::Distribution;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::Card;

use super::{state::GameState, GameConfig, GameConfigOverride};

const GAME_ID_LENGTH: usize = 16;

type GameId = String;

#[derive(Debug, Clone)]
pub struct GameIndex {
    index: HashMap<GameId, Arc<Mutex<GameState>>>,
    default_config: GameConfig,
}

/// This struct represents the list of games currently being played on the server.
impl GameIndex {
    pub fn new(default_config: GameConfig) -> Self {
        GameIndex {
            index: HashMap::new(),
            default_config,
        }
    }

    fn generate_game_id(&self) -> GameId {
        let mut id: GameId;
        loop {
            id = rand::distr::Alphanumeric
                .sample_iter(&mut rand::rng())
                .take(GAME_ID_LENGTH)
                .map(char::from)
                .collect();
            if !self.index.contains_key(&id) {
                break;
            }
        }
        id
    }

    /// Creates a new game with the default configuration.
    pub fn create_game(&mut self, decks: Vec<Vec<Card>>) -> GameId {
        let game_id = self.generate_game_id();
        self.index.insert(
            game_id.clone(),
            Arc::new(Mutex::new(GameState::new(
                self.default_config.clone(),
                decks,
            ))),
        );
        game_id
    }

    /// Creates a new game with the specified configuration override.
    pub fn create_game_with_config_override(
        &mut self,
        decks: Vec<Vec<Card>>,
        config_override: GameConfigOverride,
    ) -> GameId {
        let game_id = self.generate_game_id();
        let mut config = self.default_config.clone();
        config.apply_override(config_override);
        self.index.insert(
            game_id.clone(),
            Arc::new(Mutex::new(GameState::new(config, decks))),
        );
        game_id
    }

    /// Retrieves a game by its ID.
    pub fn get_game(&self, game_id: &GameId) -> Option<MutexGuard<GameState>> {
        self.index.get(game_id)?.lock().ok()
    }

    /// Removes a game by its ID.
    pub fn remove_game(&mut self, game_id: &GameId) {
        self.index.remove(game_id);
    }
}
