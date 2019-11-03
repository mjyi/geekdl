use geekdl::*;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 3 {
    //     panic!("please input your account and password.");
    // }
    // let account = args[1].clone();
    // let password = args[2].clone();

    // let api = GeekClient::new(account, password);

    // if let Err(log_e) = api.login().await {
    //     println!("{}", log_e);
    // }

    // if let Err(e) = api.get_course_all().await {
    //     println!("all {}", e);
    // }

    // if let Err(e) = api.get_post_list(126).await {
    //     println!("list: {}", e);
    // }

    // if let Err(e) = api.get_post_content(79433).await {
    //     println!("content: {}", e);
    // }
    //
    // std::fs::create_dir_all("some/dir").unwrap();
    // for_test().await;
    // return;

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::Query {
            account,
            password,
            country,
        } => {
            cmd::query::run(account, password, country).await.unwrap();
        }
    }
}

async fn for_test() {
    let url =
        "https://static001.geekbang.org/resource/image/8e/d3/8e603e3d795fc0ab2698f6f5eabf14d3.jpg";
    let r = utils::fetch_image(url, "").await;
}
