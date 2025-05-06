use rocket::{fs::NamedFile, State};
use rocket_okapi::openapi;
use std::{path::Path, sync::Arc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub name: String,
    pub icon: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CategoryJSON {
    pub categories: Vec<Category>,
    pub types: Vec<String>,
}

/// Returns a list of all the known categories in json format
#[openapi(tag = "Categories")]
#[get("/categories")]
pub async fn get_categories(categories: &State<Arc<CategoryJSON>>) -> Option<String> {
    serde_json::to_string(&categories.categories).ok()
}

/// Returns a list of all the known types in json format
#[openapi(tag = "Categories")]
#[get("/types")]
pub async fn get_types(categories: &State<Arc<CategoryJSON>>) -> Option<String> {
    serde_json::to_string(&categories.types).ok()
}

/// Returns a list of all the known categories and types in json format
#[openapi(tag = "Categories")]
#[get("/categories_and_types")]
pub async fn get_categories_and_types(categories: &State<Arc<CategoryJSON>>) -> Option<String> {
    serde_json::to_string(categories.inner().as_ref()).ok()
}

#[openapi(tag = "Categories")]
#[get("/category/icon/<name>")]
pub async fn get_category_icon(
    name: &str,
    categories: &State<Arc<CategoryJSON>>,
) -> Option<NamedFile> {
    let icon_path = &categories
        .categories
        .iter()
        .find(|category| category.name == name)?
        .icon;
    NamedFile::open(Path::new(&format!("decks/Categories/{icon_path}")))
        .await
        .ok()
}
