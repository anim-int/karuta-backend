use rand::prelude::*;

use super::Card;
use super::DuplicatePolicy;
use super::GameConfig;

#[derive(Debug, Copy, Clone, Eq, PartialEq, serde::Serialize)]
pub enum GameContinuation {
    Continue,
    Ffa,
    End,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, serde::Serialize)]
pub enum CardGuessResult {
    Correct(GameContinuation),
    Incorrect,
}

/// This struct represents the state of a game.
/// The `boards` field contains the current boards of each player, represented by a list of their cards.
/// The `current_card_playing` field contains the index of the current card being searched.
#[derive(Debug, Clone)]
pub struct GameState {
    config: GameConfig,
    boards: Vec<Vec<Card>>,
    current_card_playing: Option<(usize, usize)>,
    game_continuation: GameContinuation,
}

impl GameState {
    pub fn new(config: GameConfig, decks: Vec<Vec<Card>>) -> Self {
        GameState {
            config,
            boards: decks,
            current_card_playing: None,
            game_continuation: GameContinuation::Continue,
        }
    }

    pub fn get_config(&self) -> &GameConfig {
        &self.config
    }

    pub fn get_boards(&self) -> &Vec<Vec<Card>> {
        &self.boards
    }

    pub fn get_boards_mut(&mut self) -> &mut Vec<Vec<Card>> {
        &mut self.boards
    }

    pub fn get_player_board(&self, player_index: usize) -> Option<&Vec<Card>> {
        self.boards.get(player_index)
    }

    pub fn get_current_card_playing(&self) -> Option<&Card> {
        let (player_index, card_index) = self.current_card_playing?;
        self.boards.get(player_index)?.get(card_index)
    }

    pub fn get_game_continuation(&self) -> &GameContinuation {
        &self.game_continuation
    }

    pub fn has_game_ended(&self) -> bool {
        self.game_continuation == GameContinuation::End
    }

    /// Choose a card to become the current card being searched.
    pub fn play_card(&mut self) -> Result<(), &str> {
        if self.current_card_playing.is_some() {
            Err("Card already being played")
        } else if self.game_continuation == GameContinuation::End {
            Err("Game already ended")
        } else {
            // Pick a random card from the board
            let player_index = rand::rng().random_range(0..self.boards.len());
            let player_board = &self.boards[player_index];
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
                    card.anime == card_guess.anime && card.number == card_guess.number
                }
                DuplicatePolicy::MatchCard => card == &card_guess,
            };
            if result {
                Ok(CardGuessResult::Correct((self.guess_correctly())?))
            } else {
                Ok(CardGuessResult::Incorrect)
            }
        }
    }

    fn guess_correctly(&mut self) -> Result<GameContinuation, &str> {
        let (player_index, card_index) = self.current_card_playing.unwrap();
        self.boards[player_index].remove(card_index);
        self.current_card_playing = None;
        self.game_continuation = if self.boards[player_index].is_empty() {
            self.boards.remove(player_index);
            if self.config.has_ffa {
                for board in &self.boards {
                    if !board.is_empty() {
                        return Ok(GameContinuation::Ffa);
                    }
                }
                GameContinuation::End
            } else {
                GameContinuation::End
            }
        } else {
            GameContinuation::Continue
        };
        Ok(self.game_continuation)
    }
}
