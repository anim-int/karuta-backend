use rand::prelude::*;

use super::Card;
use super::DuplicatePolicy;
use super::GameConfig;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameContinuation {
    Continue,
    Ffa,
    End,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CardGuessResult {
    Correct(GameContinuation),
    Incorrect,
}

/// This struct represents the state of a game.
/// The state field contains the current boards of each player, represented by a list of their cards.
/// The current_card_playing field contains the index of the current card being searched.
#[derive(Debug, Clone)]
pub struct GameState {
    config: GameConfig,
    state: Vec<Vec<Card>>,
    current_card_playing: Option<(usize, usize)>,
}

impl GameState {
    pub fn new(config: GameConfig, decks: Vec<Vec<Card>>) -> Self {
        GameState {
            config,
            state: decks,
            current_card_playing: None,
        }
    }

    pub fn get_config(&self) -> &GameConfig {
        &self.config
    }

    pub fn get_state(&self) -> &Vec<Vec<Card>> {
        &self.state
    }

    pub fn get_player_board(&self, player_index: usize) -> Option<&Vec<Card>> {
        self.state.get(player_index)
    }

    pub fn get_player_board_mut(&mut self, player_index: usize) -> Option<&mut Vec<Card>> {
        self.state.get_mut(player_index)
    }

    pub fn get_current_card_playing(&self) -> Option<&Card> {
        let (player_index, card_index) = self.current_card_playing?;
        self.state.get(player_index)?.get(card_index)
    }

    /// Choose a card to become the current card being searched.
    pub fn play_card(&mut self) -> Result<(), &str> {
        if self.current_card_playing.is_some() {
            Err("Card already being played")
        } else {
            // Pick a random card from the board
            let player_index = rand::rng().random_range(0..self.state.len());
            let player_board = &self.state[player_index];
            let card_index = rand::rng().random_range(0..player_board.len());
            self.current_card_playing = Some((player_index, card_index));
            Ok(())
        }
    }

    /// Checks if the guessed card matches the current card being searched according to the game's duplicate card policy.
    /// If the guessed card matches the current card, the current card is removed from the board.
    pub fn guess_card(&mut self, card_guess: Card) -> Result<CardGuessResult, &str> {
        if self.current_card_playing.is_none() {
            Err("No card being played")
        } else {
            let card = self.get_current_card_playing().unwrap();
            let result = match self.config.duplicate_policy {
                DuplicatePolicy::MatchAnime => card.anime == card_guess.anime,
                DuplicatePolicy::MatchMusic => {
                    card.anime == card_guess.anime && card.type_ == card_guess.type_
                }
                DuplicatePolicy::MatchCard => card == &card_guess,
            };
            if result {
                self.guess_correctly()
            } else {
                Ok(CardGuessResult::Incorrect)
            }
        }
    }

    fn guess_correctly(&mut self) -> Result<CardGuessResult, &str> {
        let (player_index, card_index) = self.current_card_playing.unwrap();
        self.state[player_index].remove(card_index);
        self.current_card_playing = None;
        if self.state[player_index].is_empty() {
            self.state.remove(player_index);
            Ok(CardGuessResult::Correct(if self.config.has_ffa {
                GameContinuation::Ffa
            } else {
                GameContinuation::End
            }))
        } else {
            Ok(CardGuessResult::Correct(GameContinuation::Continue))
        }
    }
}
