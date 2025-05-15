#[macro_use]
extern crate rocket;

mod categories;
mod config;
mod cors;
mod deck;
mod game;
mod git_repos;
mod theme;

use categories::*;
use config::*;
use deck::*;
use game::*;
use git_repos::GitSource;
use index::GameIndex;
use theme::*;

use cors::*;

use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

use reqwest;

use std::sync::{Arc, RwLock};

const DEFAULT_CONFIG: GameConfig = GameConfig {
    has_ffa: true,
    duplicate_policy: DuplicatePolicy::MatchMusic,
};

#[launch]
fn rocket() -> _ {
    let global_config = load_global_config();
    // Load every deck indexes as GitSources using the links in the config and clone them locally
    let deck_indexes_source_index = global_config
        .sources
        .iter()
        .map(|source| GitSource::parse_url(source).unwrap())
        .map(|source| {
            source.clone_to_local(Some(&global_config.decks_directory));
            source
        })
        .collect::<Vec<GitSource>>();
    // Load decks from submodules of deck indexes
    let deck_source_index = deck_indexes_source_index
        .iter()
        .map(|source| {
            let (user, repo) = match source {
                GitSource::GitHub { user, repo } => (user, repo),
                GitSource::GitLab { user, repo } => (user, repo),
                GitSource::Sourcehut { user, repo } => (user, repo),
            };
            DeckSource::from_gitmodules(format!(
                "{}/{}_{}",
                global_config.decks_directory, user, repo
            ))
            .into_iter()
        })
        .flatten()
        .collect::<std::collections::HashSet<DeckSource>>() // Remove duplicates
        .into_iter()
        .collect::<Vec<DeckSource>>();
    let decks = deck_source_index
        .iter()
        .map(|source| {
            let response = reqwest::blocking::get(source.get_deck_json_url()).unwrap();
            (source.clone(), response.json::<Deck>().unwrap())
        })
        .collect::<Vec<(DeckSource, Deck)>>();

    let categories: CategoriesJSON = serde_json::from_reader(
        std::fs::File::open(format!(
            "{}/categories.json",
            global_config.categories_directory
        ))
        .unwrap(),
    )
    .unwrap();

    let game_index = GameIndex::new(DEFAULT_CONFIG);

    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            openapi_get_routes![
                deck_metadata,
                deck_names,
                theme_names,
                get_visual,
                get_sound,
                get_cover,
                get_theme,
                get_categories,
                get_types,
                get_categories_and_types,
                get_category_icon,
                create_game,
                create_1v1_game,
                play_card,
                guess_card,
                get_game_config,
                get_state,
                get_boards,
                get_player_board,
                move_card,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(Arc::new(decks))
        .manage(Arc::new(categories))
        .manage(Arc::new(RwLock::new(game_index)))
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::{load_global_config, CategoriesJSON, Deck};

    #[test]
    pub fn get_decks() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::deck_names)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        for deck_name in response.into_string().unwrap().lines() {
            let response = client.get(uri!(super::deck_metadata(deck_name))).dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }

    #[test]
    fn visual_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::deck_names)).dispatch();
        let deck_names = response
            .into_string()
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let decks = deck_names
            .iter()
            .map(|deck_name| {
                let response = client.get(uri!(super::deck_metadata(deck_name))).dispatch();
                serde_json::from_str(&response.into_string().unwrap()).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            for card in deck.cards {
                let response = client
                    .get(uri!(super::get_visual(deck.name.clone(), card.id)))
                    .dispatch();
                assert_eq!(response.status(), Status { code: 302 });
            }
        }
    }

    #[test]
    fn audio_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::deck_names)).dispatch();
        let deck_names = response
            .into_string()
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let decks = deck_names
            .iter()
            .map(|deck_name| {
                let response = client.get(uri!(super::deck_metadata(deck_name))).dispatch();
                serde_json::from_str(&response.into_string().unwrap()).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            for card in deck.cards {
                let response = client
                    .get(uri!(super::get_sound(deck.name.clone(), card.id)))
                    .dispatch();
                assert_eq!(response.status(), Status { code: 302 });
            }
        }
    }

    #[test]
    fn cover_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::deck_names)).dispatch();
        let deck_names = response
            .into_string()
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let decks = deck_names
            .iter()
            .map(|deck_name| {
                let response = client.get(uri!(super::deck_metadata(deck_name))).dispatch();
                serde_json::from_str(&response.into_string().unwrap()).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            let response = client
                .get(uri!(super::get_cover(deck.name.clone())))
                .dispatch();
            assert_eq!(response.status(), Status { code: 302 });
        }
    }

    #[test]
    fn category_files_integrity() {
        let global_config = load_global_config();
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let categories: CategoriesJSON = serde_json::from_reader(
            std::fs::File::open(format!(
                "{}/categories.json",
                global_config.categories_directory
            ))
            .unwrap(),
        )
        .unwrap();

        for category in categories.categories {
            let response = client
                .get(uri!(super::get_category_icon(category.name)))
                .dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }
}
