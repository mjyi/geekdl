use crate::{errors::Error, utils, Article, Column, GeekClient};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    io::prelude::*,
    thread,
    time::Duration,
};

pub async fn run(account: String, password: String, country: String) -> Result<(), Error> {
    let client = GeekClient::new(account, password, country);
    if let Err(e) = client.login().await {
        println!("{}", e);
    }

    let courses = client.get_column_all().await?;

    let mut empty = true;
    let mut output = String::new();
    let mut c_map: HashMap<i32, Column> = HashMap::new();
    for data in courses {
        if !data.is_empty() {
            empty = false;
            let line = format!("{:=^1$}\n", data.title, 20);
            output.push_str(&line);
        }
        for cv in data.list {
            let line = format!(
                "{0: <8} {1: <8} {2:}\n",
                cv.extra.column_id, cv.extra.author_name, cv.title
            );
            output.push_str(&line);
            c_map.insert(cv.extra.column_id, cv);
        }
    }

    if empty {
        println!("No Courses!");
        return Ok(());
    }

    println!("{}", output);

    // Stdio Input
    println!("Please enter the id, separated by `,` : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // remove `\n`
    input.pop();

    let ids: Vec<i32> = input
        .split(",")
        .into_iter()
        .filter(|&x| x != "")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    for id in ids {
        if let Some(ref mut col) = c_map.get(&id) {
            println!("Get id {}", id);
            let mut articles = client
                .get_articles(col.extra.column_id, col.extra.last_aid)
                .await?;
            println!("after get_articles {}", articles.len());
            for article in &mut articles {
                println!(
                    "Prepare to download {}, {}",
                    article.article_title, article.id
                );
                let mut content = None;
                thread::sleep(Duration::from_secs(3));
                match client.get_article_content(article.id).await {
                    Ok(c) => content = Some(c),
                    Err(e) => println!("{:?}", e),
                };
                article.content = content;
            }
            generate_column(col, &mut articles).unwrap_or(());
            println!("after generate");
        } else {
            println!("{} Not Found", id);
        }
    }
    Ok(())
}

// 已下载文章内容写入单文件
fn generate_column(column: &Column, articles: &mut Vec<Article>) -> Result<(), Error> {
    let target_dir = column.title.clone();
    fs::create_dir(&target_dir).unwrap_or(());

    let mut data_json = File::create(&format!("{}/data.json", target_dir))?;

    let full = serde_json::to_string(&articles).unwrap_or("".to_string());

    println!("full len {}", full.len());
    data_json.write_all(&full.as_bytes()).unwrap_or(());

    // let mut source = File::create(&format!("{}/source.html", target_dir))?;
    // for article in articles {
    //  if let Some(ref content) = article.content {
    //      let content = format!("<h1>{}</h1>\n{}", article.article_title, content.article_content);
    //      source.write(content.as_bytes())?;
    // }
    // }
    Ok(())
}

pub async fn replace_img_tags(content: String, dist: String) -> String {
    let mut content2 = content.clone();
    //    let mut imgs = Vec::new();
    let re = Regex::new(r#"<img src="(?P<img>.*?)""#).unwrap();
    for cap in re.captures_iter(&content) {
        let img = &cap["img"];
        match utils::fetch_image(img, &dist).await {
            Ok(replaced) => {
                content2 = content2.replace(img, &replaced);
            }
            Err(e) => println!("{}", e),
        }
    }

    content2
}
