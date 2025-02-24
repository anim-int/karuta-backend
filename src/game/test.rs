use crate::state::CardGuessResult;

use crate::deck::*;
use crate::*;

fn create_sample_cards() -> Vec<Card> {
    vec![
        Card {
            anime: "Sample Anime 1".to_string(),
            type_: "OP".to_string(),
            visual: "Sample Visual 1".to_string(),
            audio: "Sample Audio 1".to_string(),
        },
        Card {
            anime: "Sample Anime 2".to_string(),
            type_: "OP".to_string(),
            visual: "Sample Visual 2".to_string(),
            audio: "Sample Audio 2".to_string(),
        },
        Card {
            anime: "Sample Anime 1".to_string(),
            type_: "ED".to_string(),
            visual: "Sample Visual 3".to_string(),
            audio: "Sample Audio 3".to_string(),
        },
        Card {
            anime: "Sample Anime 1".to_string(),
            type_: "OP".to_string(),
            visual: "Sample Visual 4".to_string(),
            audio: "Sample Audio 4".to_string(),
        },
    ]
}

#[test]
fn create_game() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()]]);

    let game = game_index.get_game(&game_id).unwrap();
    assert_eq!(game.get_state().len(), 1);
}

#[test]
fn create_game_and_play() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()]]);

    let game = game_index.get_game_mut(&game_id).unwrap();
    game.play_card().unwrap();
    assert_eq!(game.get_current_card_playing().unwrap(), &cards[0])
}

#[test]
fn play_game_with_match_anime() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchAnime,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()]]);

    let game = game_index.get_game_mut(&game_id).unwrap();
    game.play_card().unwrap();
    assert_eq!(
        game.guess_card(cards[1].clone()).unwrap(),
        CardGuessResult::Incorrect,
    );
    assert_eq!(
        game.guess_card(cards[2].clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::End),
    );
}

#[test]
fn play_game_with_match_music() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()]]);

    let game = game_index.get_game_mut(&game_id).unwrap();
    game.play_card().unwrap();
    assert_eq!(
        game.guess_card(cards[1].clone()).unwrap(),
        CardGuessResult::Incorrect,
    );
    assert_eq!(
        game.guess_card(cards[2].clone()).unwrap(),
        CardGuessResult::Incorrect,
    );
    assert_eq!(
        game.guess_card(cards[3].clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::End),
    );
}

#[test]
fn play_game_with_match_card() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchCard,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()]]);

    let game = game_index.get_game_mut(&game_id).unwrap();
    game.play_card().unwrap();
    assert_eq!(
        game.guess_card(cards[1].clone()).unwrap(),
        CardGuessResult::Incorrect,
    );
    assert_eq!(
        game.guess_card(cards[2].clone()).unwrap(),
        CardGuessResult::Incorrect,
    );
    assert_eq!(
        game.guess_card(cards[3].clone()).unwrap(),
        CardGuessResult::Incorrect,
    );
    assert_eq!(
        game.guess_card(cards[0].clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::End),
    );
}
