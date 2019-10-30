#[macro_use]
extern crate log;
use geekdl::api::*;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("please input your account and password.");
    }
    let account = args[1].clone();
    let password = args[2].clone();

    let api = GeekClient::new(account, password);

    let _ = api.login().await;

    let _ = api.get_course_all().await;

    // let _ = api.get_post_list(126).await;

    // let _ = api.get_post_content(79433).await;
}
