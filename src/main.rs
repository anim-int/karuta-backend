#[macro_use]
extern crate rocket;

mod deck;
use deck::*;

mod theme;
use theme::*;

mod categories;
use categories::*;

mod cors;
use cors::*;

use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use std::sync::Arc;

#[launch]
fn rocket() -> _ {
    let decks = std::fs::read_dir("decks/Decks")
        .unwrap()
        .map(|path| {
            let reader = std::fs::File::open(path.unwrap().path()).unwrap();
            serde_json::from_reader(reader).unwrap()
        })
        .collect::<Vec<Deck>>();
    let categories: CategoryJSON =
        serde_json::from_reader(std::fs::File::open("decks/Categories/Categories.json").unwrap())
            .unwrap();

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
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::{CategoryJSON, Deck};

    #[test]
    fn get_decks() {
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
        let decks = std::fs::read_dir("decks/Decks")
            .unwrap()
            .map(|path| {
                let reader = std::fs::File::open(path.unwrap().path()).unwrap();
                serde_json::from_reader(reader).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            for card in deck.cards {
                let response = client.get(uri!(super::get_visual(card.visual))).dispatch();
                assert_eq!(response.status(), Status::Ok);
            }
        }
    }

    #[test]
    fn audio_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let decks = std::fs::read_dir("decks/Decks")
            .unwrap()
            .map(|path| {
                let reader = std::fs::File::open(path.unwrap().path()).unwrap();
                serde_json::from_reader(reader).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            for card in deck.cards {
                let response = client.get(uri!(super::get_sound(card.audio))).dispatch();
                assert_eq!(response.status(), Status::Ok);
            }
        }
    }

    #[test]
    fn category_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let categories: CategoryJSON = serde_json::from_reader(
            std::fs::File::open("decks/Categories/Categories.json").unwrap(),
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
