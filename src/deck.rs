use schemars::JsonSchema;
use serde;

use rocket::{fs::NamedFile, serde::json::Json, State};
use rocket_okapi::openapi;
use std::{path::Path, sync::Arc};

#[derive(Debug)]
pub enum DeckSource {
    GitHub { user: String, repo: String },
    GitLab { user: String, repo: String },
    Sourcehut { user: String, repo: String },
}

impl DeckSource {
    pub fn parse_url(url: &str) -> Option<DeckSource> {
        if url.starts_with("https://github.com/") {
            let parts: Vec<&str> = url.split('/').collect();
            Some(DeckSource::GitHub {
                user: parts[3].to_string(),
                repo: parts[4].trim_end_matches(".git").to_string(),
            })
        } else if url.starts_with("https://gitlab.com/") {
            let parts: Vec<&str> = url.split('/').collect();
            Some(DeckSource::GitLab {
                user: parts[3].to_string(),
                repo: parts[4].trim_end_matches(".git").to_string(),
            })
        } else if url.starts_with("https://git.sr.ht/") {
            let parts: Vec<&str> = url.split('/').collect();
            Some(DeckSource::Sourcehut {
                user: parts[3].to_string(),
                repo: parts[4].to_string(),
            })
        } else {
            None
        }
    }

    pub fn from_gitmodules<P>(directory: P) -> Vec<Self>
    where
        P: AsRef<Path>,
    {
        let gitmodules = std::fs::read_to_string(directory.as_ref().join(".gitmodules")).unwrap();
        gitmodules
            .lines()
            .filter(|line| line.starts_with("\turl"))
            .map(|line| Self::parse_url(line.trim_start_matches("\turl = ")).unwrap())
            .collect()
    }

    fn get_file_url<S>(&self, path: S) -> String
    where
        S: ToString,
    {
        match self {
            DeckSource::Sourcehut { user, repo } => {
                format!(
                    "https://git.sr.ht/{}/{}/blob/main/{}",
                    user,
                    repo,
                    path.to_string()
                )
            }
            DeckSource::GitLab { user, repo } => {
                format!(
                    "https://gitlab.com/{}/{}/raw/main/{}",
                    user,
                    repo,
                    path.to_string()
                )
            }
            DeckSource::GitHub { user, repo } => format!(
                "https://raw.githubusercontent.com/{}/{}/refs/heads/main/{}",
                user,
                repo,
                path.to_string()
            ),
        }
    }

    pub fn get_deck_json_url(&self) -> String {
        self.get_file_url("deck.json")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Deck {
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub author: String,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Card {
    pub id: u32,
    pub anime: String,
    pub number: String,
    pub title: String,
    pub authors: String,
    pub image: String,
    pub audio: String,
    pub anilist_id: u32,
}

#[openapi(tag = "Decks")]
#[get("/deck/metadata/<name>")]
pub async fn deck_metadata(decks: &State<Arc<Vec<Deck>>>, name: &str) -> Option<Json<Deck>> {
    decks
        .iter()
        .find(|deck| deck.name == name)
        .map(|deck| Json(deck.clone()))
}

#[openapi(tag = "Decks")]
#[get("/deck/names")]
pub fn deck_names(decks: &State<Arc<Vec<Deck>>>) -> String {
    decks.iter().map(|deck| deck.name.clone() + "\n").collect()
}

#[openapi(tag = "Decks")]
#[get("/visual/<name>")]
pub async fn get_visual(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Visuals/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Decks")]
#[get("/sound/<name>")]
pub async fn get_sound(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Sounds/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Decks")]
#[get("/deck/cover/<name>")]
pub async fn get_cover(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Covers/{name}")))
        .await
        .ok()
}
