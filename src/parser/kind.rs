use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag="kind")]
pub enum Frontmatter {
    Book(Book),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    title: String,
    slug: String,
    created: String,
    updated: String,

    started: Option<String>,
    finished: Option<String>,
    status: BookStatus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BookStatus {
    Todo,
    Doing,
    Done,
}
