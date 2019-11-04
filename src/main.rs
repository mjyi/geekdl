use geekdl::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
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
        _ => (),
    }
}

async fn for_test() {
    let url =
        "https://static001.geekbang.org/resource/image/8e/d3/8e603e3d795fc0ab2698f6f5eabf14d3.jpg";
    let r = utils::fetch_image(url, "").await;
}
