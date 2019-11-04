use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "极客时间课程下载工具")]
pub enum Opt {
    Query {
        #[structopt(long, short)]
        account: String,
        #[structopt(long, short)]
        password: String,
        #[structopt(long, short = "c", help = "the code of country", default_value = "86")]
        country: String,
    },
    Gen {
        #[structopt(long, short = "i")]
        input: String,
        #[structopt(long, short = "o", help = "the output name")]
        out: String,
    },
}
