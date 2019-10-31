use crate::{errors::Error, GeekClient, Column, ColumnsData, Content};
use std::io::{self, Read};

pub async fn run(account: String, password: String, country: String) -> Result<(), Error> {
    let client = GeekClient::new(account, password, country);
    if let Err(e) = client.login().await {
        println!("{}", e);
    }
    println!("Login Success");

    let courses = client.get_course_all().await?;

    show_courses(courses);

    Ok(())
}

fn show_courses(courses: Vec<ColumnsData>) {
    let mut empty = true;
    let mut output = String::new();

    for data in courses {
        if !data.is_empty() {
            empty = false;
            let line = format!("{:=^1$}\n", data.title, 20);
            output.push_str(&line);
        }
        for cv in data.list {
            let line = format!(
                "{0: <6} | {1: <6} | {2:}\n", 
                cv.extra.column_id, 
                cv.extra.author_name, 
                cv.title);
            output.push_str(&line);
        }
    }

    if empty {
        println!("No Courses!");
    } else {
        println!("{}", output);
    }

    input_command().unwrap();
}

fn input_command() -> Result<(), Error> {
    println!("请输入课程 id，用 `,` 隔开：");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // TODO: remove `\n`
    let mut ids:Vec<i32> = input.split(",").into_iter().map(|s| s.parse::<i32>().unwrap_or(0)).collect();

    println!("{:?}", ids);

    Ok(())
}
