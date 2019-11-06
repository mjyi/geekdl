use crate::{errors::Error, model::Article};
use std::{fs::File, io::prelude::*, io::BufReader};

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
