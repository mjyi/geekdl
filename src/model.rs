use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnsData {
    pub id: i32,
    pub title: String,
    pub list: Vec<Column>,
}

impl ColumnsData {
    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub title: String,
    #[serde(rename = "type")]
    pub rtype: String,
    pub extra: Extra,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extra {
    pub column_id: i32,
    pub column_title: String,
    pub view_article_count: i32,
    pub article_count: i32,
    pub author_name: String,
    pub author_intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub article_title: String,
    pub audio_download_url: String,
    pub chapter_id: String,
    pub column_id: i32,
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub chapter_id: String,
    pub column_id: String,
    pub article_title: String,
    pub article_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Comment {}
