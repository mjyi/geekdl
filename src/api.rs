use reqwest::{
    header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, HOST, REFERER, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// 登录
const LOGIN_URI: &'static str = "https://account.geekbang.org/account/ticket/login";
// 订阅课程
const COURSES_URI: &'static str = "https://time.geekbang.org/serv/v1/my/products/all";
// 课程章节列表
const COURSE_LIST_URI: &'static str = "https://time.geekbang.org/serv/v1/column/articles";
// 课程内容
const POST_CONTENT_URI: &'static str = "https://time.geekbang.org/serv/v1/article";
// 评论内容
const COMMENT_URI: &'static str = "https://time.geekbang.org/serv/v1/comments";

pub struct GeekClient {
    account: String,
    password: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Course {
    column_id: i32,
    name: String,

}


// 登录参数
#[derive(Debug, Serialize, Deserialize)]
struct Login {
    country: String,
    cellphone: String,
    password: String,
    captcha: String,
    remember: i32,
    platform: i32,
    #[serde(rename = "appid")]
    app_id: i32,
}

impl GeekClient {
    pub fn new(account: String, password: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:70.0) Gecko/20100101 Firefox/70.0"
            .parse()
            .unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("error of build the client");

        GeekClient {
            account,
            password,
            client,
        }
    }

    pub async fn login(&self) -> Result<(), reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json, text/plain, */*".parse().unwrap());
        headers.insert(
            ACCEPT_LANGUAGE,
            "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2"
            .parse()
            .unwrap(),
        );
        headers.insert(HOST, "account.geekbang.org".parse().unwrap());
        headers.insert(
            REFERER,
            "https://account.geekbang.org/signin?redirect=https%3A%2F%2Fwww.geekbang.org%2F"
            .parse()
            .unwrap(),
        );

        let data = json!({
            "country": "86",
            "cellphone": self.account.clone(),
            "password": self.password.clone(),
            "captcha":"",
            "remember": 1,
            "platform": 3,
            "appid": 1,
        });

        // let data = Login {
        //     country: "86".to_owned(),
        //     cellphone: self.account.clone(),
        //     password: self.password.clone(),
        //     captcha: "".to_owned(),
        //     remember: 1,
        //     platform: 3,
        //     app_id: 1,
        // };
        let req: Value = self
            .client
            .post(LOGIN_URI)
            .headers(headers)
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        // println!("{:#?}", req);
        Ok(())
    }

    pub async fn get_course_all(&self) -> Result<(), reqwest::Error> {
        let req: Value = self.client
            .post(COURSES_URI)
            .header(REFERER, "https://account.geekbang.org/dashboard/buy")
            .send()
            .await?
            .json()
            .await?;

        println!("{}", req.to_string());
        Ok(())
    }

    pub async fn get_post_list(&self, course_id: i32) -> Result<(), reqwest::Error> {
        let data = json!({
            "cid": course_id.to_string(),
            "size": 1000,
            "prev": 0,
            "order": "newest"
        });

        let req: Value = self
            .client
            .post(COURSE_LIST_URI)
            .header(REFERER, format!("https://time.geekbang.org/column/{}", course_id))
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", req);

        Ok(())
    }

    pub async fn get_post_content(&self, post_id: i32) -> Result<(), reqwest::Error> {
        let data = json!({
            "id": post_id,
        });

        let req: Value = self
            .client
            .post(POST_CONTENT_URI)
            .header(REFERER, format!("https://time.geekbang.org/column/article/{}", post_id))
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", req);

        Ok(())
    }

    pub async fn get_post_comment(&self, post_id: i32) -> Result<(), reqwest::Error> {
        let data = json!({
            "aid": post_id,
            "prev":0,
        });

        let req: Value = self.client
            .post(COMMENT_URI)
            .header(REFERER, format!("https://time.geekbang.org/column/article/{}", post_id))
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", req);

        Ok(())
    }
}

