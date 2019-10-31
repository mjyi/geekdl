use structopt::StructOpt;
use geekdl::*;
use std::env;

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

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::Query{ account, password, area } => {
            println!("{}, {}, {}", account, password, area);
        },
    }

}

