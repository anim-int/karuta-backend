#![allow(unused)]
use crate::*;

use crate::deck::*;
use crate::state::CardGuessResult;

fn create_sample_cards() -> Vec<Card> {
    vec![
        Card {
            id: 0,
            anime: "Sample Anime 1".to_string(),
            number: "OP".to_string(),
            image: "Sample Visual 1".to_string(),
            audio: "Sample Audio 1".to_string(),
            title: "Sample Title 1".to_string(),
            authors: "Sample Author 1".to_string(),
            anilist_id: 1,
        },
        Card {
            id: 1,
            anime: "Sample Anime 2".to_string(),
            number: "OP".to_string(),
            image: "Sample Visual 2".to_string(),
            audio: "Sample Audio 2".to_string(),
            title: "Sample Title 2".to_string(),
            authors: "Sample Author 2".to_string(),
            anilist_id: 2,
        },
        Card {
            id: 2,
            anime: "Sample Anime 1".to_string(),
            number: "ED".to_string(),
            image: "Sample Visual 3".to_string(),
            audio: "Sample Audio 3".to_string(),
            title: "Sample Title 3".to_string(),
            authors: "Sample Author 3".to_string(),
            anilist_id: 3,
        },
        Card {
            id: 3,
            anime: "Sample Anime 1".to_string(),
            number: "OP".to_string(),
            image: "Sample Visual 4".to_string(),
            audio: "Sample Audio 4".to_string(),
            title: "Sample Title 1".to_string(),
            authors: "Sample Author 1".to_string(),
            anilist_id: 4,
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
    assert_eq!(game.get_boards().len(), 1);
}

#[test]
fn create_simple_game_and_play() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()], vec![cards[0].clone()]]);

    let mut game = game_index.get_game(&game_id).unwrap();
    game.play_card().unwrap();

    assert_eq!(game.get_current_card_playing().unwrap(), &cards[0]);

    assert_eq!(
        game.guess_card(cards[0].clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::End),
    );

    assert!(game.has_game_ended());
}

#[test]
fn create_game_with_ffa_and_play() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: true,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()], vec![cards[0].clone()]]);

    let mut game = game_index.get_game(&game_id).unwrap();
    game.play_card().unwrap();

    assert_eq!(game.get_current_card_playing().unwrap(), &cards[0]);

    assert_eq!(
        game.guess_card(cards[0].clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::Ffa),
    );

    game.play_card().unwrap();

    assert_eq!(
        game.guess_card(cards[0].clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::End),
    );

    assert!(game.has_game_ended());
}

#[test]
fn play_game_with_match_anime() {
    let cards = create_sample_cards();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchAnime,
    });

    let game_id = game_index.create_game(vec![vec![cards[0].clone()]]);

    let mut game = game_index.get_game(&game_id).unwrap();
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

    let mut game = game_index.get_game(&game_id).unwrap();
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

    let mut game = game_index.get_game(&game_id).unwrap();
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
