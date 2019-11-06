use crate::{errors::Error, model::Article};
use std::ptr;

use std::{
    cmp::Ordering,
    fs::{self, File},
    io::prelude::*,
    io::{self, BufReader},
};

pub fn run(input: &str, out: &str) -> Result<(), Error> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut articles: Vec<Article> = serde_json::from_reader(reader)?;

    // articles.reverse();
    articles.sort_by(|a, b| a.chapter_id.cmp(&b.chapter_id));

    let mut source = File::create(out)?;
    for article in articles {
        if let Some(ref content) = article.content {
            let content = format!(
                "<h1>{}</h1>\n{}",
                article.article_title, content.article_content
            );
            source.write(content.as_bytes())?;
        }
    }

    Ok(())
}

fn sort(mut articles: Vec<Article>) -> Vec<Article> {
    if articles.is_empty() {
        return articles;
    }
    for i in 1..articles.len() {
        let value = articles[i].clone();
        let mut j = (i - 1) as i32;
        while j >= 0 {
            if articles[j as usize].chapter_id > value.chapter_id {
                articles.swap(j as usize + 1, j as usize);
            } else if articles[j as usize].id > value.id {
                articles.swap(j as usize + 1, j as usize);
            }
            j -= 1;
        }
    }
    articles
}
