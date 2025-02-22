use crate::state::CardGuessResult;

use crate::deck::*;
#[cfg(test)]
use crate::*;

const DEFAULT_CONFIG: super::GameConfig = super::GameConfig {
    has_ffa: true,
    duplicate_policy: super::DuplicatePolicy::MatchMusic,
};

fn create_sample_decks_match_anime() -> Vec<Deck> {
    vec![
        Deck {
            name: "Sample Deck 1".to_string(),
            category: "Sample Category 1".to_string(),
            type_: "Sample Type 1".to_string(),
            cover: "Sample Cover 1".to_string(),
            cards: vec![Card {
                anime: "Sample Anime 1".to_string(),
                type_: "OP".to_string(),
                visual: "Sample Visual 1".to_string(),
                audio: "Sample Audio 1".to_string(),
            }],
        },
        Deck {
            name: "Sample Deck 2".to_string(),
            category: "Sample Category 2".to_string(),
            type_: "Sample Type 2".to_string(),
            cover: "Sample Cover 2".to_string(),
            cards: vec![Card {
                anime: "Sample Anime 1".to_string(),
                type_: "ED".to_string(),
                visual: "Sample Visual 2".to_string(),
                audio: "Sample Audio 2".to_string(),
            }],
        },
    ]
}

fn create_sample_decks_match_music() -> Vec<Deck> {
    vec![
        Deck {
            name: "Sample Deck 1".to_string(),
            category: "Sample Category 1".to_string(),
            type_: "Sample Type 1".to_string(),
            cover: "Sample Cover 1".to_string(),
            cards: vec![Card {
                anime: "Sample Anime 1".to_string(),
                type_: "OP".to_string(),
                visual: "Sample Visual 1".to_string(),
                audio: "Sample Audio 1".to_string(),
            }],
        },
        Deck {
            name: "Sample Deck 2".to_string(),
            category: "Sample Category 2".to_string(),
            type_: "Sample Type 2".to_string(),
            cover: "Sample Cover 2".to_string(),
            cards: vec![Card {
                anime: "Sample Anime 1".to_string(),
                type_: "OP".to_string(),
                visual: "Sample Visual 2".to_string(),
                audio: "Sample Audio 2".to_string(),
            }],
        },
    ]
}

fn create_sample_decks_match_card() -> Vec<Deck> {
    vec![
        Deck {
            name: "Sample Deck 1".to_string(),
            category: "Sample Category 1".to_string(),
            type_: "Sample Type 1".to_string(),
            cover: "Sample Cover 1".to_string(),
            cards: vec![Card {
                anime: "Sample Anime 1".to_string(),
                type_: "OP".to_string(),
                visual: "Sample Visual 1".to_string(),
                audio: "Sample Audio 1".to_string(),
            }],
        },
        Deck {
            name: "Sample Deck 2".to_string(),
            category: "Sample Category 2".to_string(),
            type_: "Sample Type 2".to_string(),
            cover: "Sample Cover 2".to_string(),
            cards: vec![Card {
                anime: "Sample Anime 1".to_string(),
                type_: "OP".to_string(),
                visual: "Sample Visual 1".to_string(),
                audio: "Sample Audio 1".to_string(),
            }],
        },
    ]
}

#[test]
fn create_game() {
    let decks = create_sample_decks_match_card();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(
        decks
            .iter()
            .take(2)
            .map(|deck| deck.cards.clone())
            .collect(),
    );

    let game = game_index.get_game(&game_id).unwrap();
    assert_eq!(game.get_state().len(), 2);
}

#[test]
fn create_game_and_play() {
    let decks = create_sample_decks_match_card();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchMusic,
    });

    let game_id = game_index.create_game(
        decks
            .iter()
            .take(2)
            .map(|deck| deck.cards.clone())
            .collect(),
    );

    let game = game_index.get_game_mut(&game_id).unwrap();
    game.play_card().unwrap();
    assert_eq!(
        game.get_current_card_playing().unwrap(),
        decks[0].cards.get(0).unwrap()
    )
}

#[test]
fn create_game_with_match_anime() {
    let decks = create_sample_decks_match_anime();

    let mut game_index = GameIndex::new(GameConfig {
        has_ffa: false,
        duplicate_policy: DuplicatePolicy::MatchAnime,
    });

    let game_id = game_index.create_game(
        decks
            .iter()
            .take(2)
            .map(|deck| deck.cards.clone())
            .collect(),
    );

    let game = game_index.get_game_mut(&game_id).unwrap();
    game.play_card().unwrap();
    let guess = if game.get_current_card_playing().unwrap().type_ == "OP" {
        assert_eq!(
            game.get_current_card_playing().unwrap(),
            decks[0].cards.get(0).unwrap()
        );
        decks[1].cards.get(0).unwrap()
    } else {
        assert_eq!(
            game.get_current_card_playing().unwrap(),
            decks[1].cards.get(0).unwrap()
        );
        decks[0].cards.get(0).unwrap()
    };
    assert_eq!(
        game.guess_card(guess.clone()).unwrap(),
        CardGuessResult::Correct(state::GameContinuation::End),
    );
}
