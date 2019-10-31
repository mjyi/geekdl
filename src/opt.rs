use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "极客时间课程下载工具")]
pub enum Opt {
    Query {
        #[structopt(long, short)]
        account: String,
        #[structopt(long, short)]
        password: String,
        #[structopt(long, short = "c", help = "the code of area", default_value = "86")]
        area: String,
    },
}
