use crate::{errors::Error, GeekClient, Column, ColumnsData, Content};
use std::io;

pub async fn run(account: String, password: String, country: String) -> Result<(), Error> {
    let client = GeekClient::new(account, password, country);
    if let Err(e) = client.login().await {
        println!("{}", e);
    }
    println!("Login Success");

    let courses = client.get_course_all().await?;

    Ok(())
}


fn show_courses(courses: Vec<ColumnsData>) {

}
