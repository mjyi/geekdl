#[macro_use]
extern crate log;

use geekdl::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    debug!("{:?}", opt);

    match opt {
        Opt::Query {
            account,
            password,
            country,
        } => {
            cmd::query::run(account, password, country).await.unwrap();
        }
        Opt::Gen { input, out } => cmd::gen::run(&input, &out).unwrap(),
    }
}

