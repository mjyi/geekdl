use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnsData {
    id: i32,
    title: String,
    list: Vec<Column>,
}

impl ColumnsData {
    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    title: String,
    #[serde(rename = "type")]
    rtype: String,
    extra: Extra,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extra {
    column_id: i32,
    column_title: String,
    view_article_count: i32,
    article_count: i32,
    author_name: String,
    author_intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    id: i32,
    article_title: String,
    audio_download_url: String,
    chapter_id: String,
    column_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    chapter_id: String,
    column_id: String,
    article_title: String,
    article_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Comment {}
