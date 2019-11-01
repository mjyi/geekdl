use crate::{errors::Error, Column, Article, Content, GeekClient};
use std::{
    collections::HashMap,
    io::{self, Read},
    path::PathBuf,
    fs::File,
};
use regex::Regex;


pub async fn run(account: String, password: String, country: String) -> Result<(), Error> {
    let client = GeekClient::new(account, password, country);
    if let Err(e) = client.login().await {
        println!("{}", e);
    }
    println!("Login Success");

    let courses = client.get_column_all().await?;

    let mut empty = true;
    let mut output = String::new();
    let mut cMap: HashMap<i32, Column> = HashMap::new();
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
            cMap.insert(cv.extra.column_id, cv);
        }
    }

    if empty {
        println!("No Courses!");
        return Ok(());
    }

    println!("{}", output);

    // Stdin Input
    println!("Please enter the id, separated by `,` : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // remove `\n`
    input.pop();

    let mut ids: Vec<i32> = input
        .split(",")
        .into_iter()
        .filter(|&x| x != "")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    println!("{:?}", ids);
    for id in ids {
        if let Some(ref mut col) = cMap.get(&id) {
            let mut articles = client.get_post_list(col.extra.column_id).await?;
            for article in &mut articles {

                let content = client.get_post_content(article.id).await?;
                article.content = content;
            }
            generate_column(col, articles);
        } else {
            println!("{} Not Found", id);
        }
    }

    Ok(())
}

fn generate_column(column: &Column, articles: Vec<Article>) {

}


pub fn download_replace_img(content: String) -> String {
    let mut content = content;
    let mut imgs: Vec<String> = Vec::new();
    let re = Regex::new(r#"<img src="(?P<img>.*?)""#).unwrap();
    
    for cap in re.captures_iter(&content) {
        let img = &cap["img"];
        imgs.push(img.to_owned());
    }


    content
}



