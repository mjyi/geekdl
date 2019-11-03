use crate::{errors::Error, utils, Article, Column, GeekClient};
use regex::{Regex};
use std::{
    collections::HashMap,
    io,
};

pub async fn run(account: String, password: String, country: String) -> Result<(), Error> {
    let client = GeekClient::new(account, password, country);
    if let Err(e) = client.login().await {
        println!("{}", e);
    }
    println!("Login Success");

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

// 下载文章内容
fn generate_column(column: &Column, articles: Vec<Article>) {
    unimplemented!()
}

//pub fn into_page(title: &str, content: String) -> String {
//    format!(
//        r#"
//<!DOCTYPE html>
//<html lang="en">
//<head>
//    <meta charset="UTF-8">
//    <meta name="viewport" content="width=device-width, initial-scale=1.0">
//    <meta http-equiv="X-UA-Compatible" content="ie=edge">
//    <title>Document</title>
//</head>
//<body>
//    <h1>{}</h1>
//
//    {}
//</body>
//</html>"#,
//        title, content
//    )
//}

pub async fn replace_img_tags(content: String, dist: String) -> String {
    let mut content2 = content.clone();
//    let mut imgs = Vec::new();
    let re = Regex::new(r#"<img src="(?P<img>.*?)""#).unwrap();
    for cap in re.captures_iter(&content) {
        let img = &cap["img"];
        match utils::fetch_image(img, &dist).await {
            Ok(replaced) => {
//                let mut src = content.replace(m, &replaced);
//                std::mem::replace(&mut content,  src);
                content2 = content2.replace(img, &replaced);
            },
            Err(e) => println!("{}", e),
        }

    }

    content2
}
