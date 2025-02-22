use rocket::fs::NamedFile;
use rocket_okapi::openapi;
use std::path::Path;

#[openapi(tag = "Themes")]
#[get("/theme/names")]
pub fn theme_names() -> String {
    std::fs::read_dir("decks/Themes")
        .unwrap()
        .map(|rd| rd.unwrap().file_name().into_string().unwrap())
        .filter(|filename| filename.contains(".json"))
        .map(|s| s + "\n")
        .collect()
}

#[openapi(tag = "Themes")]
#[get("/theme/<name>")]
pub async fn get_theme(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Themes/{name}")))
        .await
        .ok()
}
