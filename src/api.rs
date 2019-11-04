use crate::{errors::Error, model::*, utils};
use reqwest::{
    header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, HOST, REFERER, USER_AGENT},
    Client,
};
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
    country: String,
    client: Client,
}

impl GeekClient {
    pub fn new(account: String, password: String, country: String) -> Self {
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
            country,
            client,
        }
    }

    pub async fn login(&self) -> Result<(), Error> {
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
            "country": self.country.clone(),
            "cellphone": self.account.clone(),
            "password": self.password.clone(),
            "captcha":"",
            "remember": 1,
            "platform": 3,
            "appid": 1,
        });

        let req: Value = self
            .client
            .post(LOGIN_URI)
            .headers(headers)
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        if is_success_code(&req) {
            Ok(())
        } else {
            Err(Error::LoginFailed(req["error"]["msg"].clone()))?
        }
    }

    pub async fn get_column_all(&self) -> Result<Vec<ColumnsData>, Error> {
        let req: Value = self
            .client
            .post(COURSES_URI)
            .header(REFERER, "https://account.geekbang.org/dashboard/buy")
            .send()
            .await?
            .json()
            .await?;

        if !is_success_code(&req) {
            return Err(Error::ResponseError(req["error"].clone()))?;
        }
        let containers = &req["data"];
        let ret: Vec<ColumnsData> = serde_json::from_value(containers.clone())?;
        Ok(ret)
    }


    pub async fn get_articles(&self, cid: i32) -> Result<Vec<Article>, Error> {
        let data = json!({
            "cid": cid.to_string(),
            "size": 1000,
            "prev": 0,
            "order": "newest"
        });

        let req: Value = self
            .client
            .post(COURSE_LIST_URI)
            .header(REFERER, format!("https://time.geekbang.org/column/{}", cid))
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        
        let _ = utils::write_to_file(&req.to_string(), "articles.json");
        if !is_success_code(&req) {
            return Err(Error::ResponseError(req["error"].clone()))?;
        }

        let ret: Vec<Article> = serde_json::from_value(req["data"]["list"].clone())?;
        Ok(ret)
    }

    pub async fn get_article_content(&self, post_id: i32) -> Result<Content, Error> {
        let data = json!({
            "id": post_id,
        });

        let req: Value = self
            .client
            .post(POST_CONTENT_URI)
            .header(
                REFERER,
                format!("https://time.geekbang.org/column/article/{}", post_id),
            )
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        if !is_success_code(&req) {
            return Err(Error::ResponseError(req["error"].clone()))?;
        }

        let content: Content = serde_json::from_value(req["data"].clone())?;
        Ok(content)
    }

    pub async fn get_post_comment(&self, post_id: i32) -> Result<(), reqwest::Error> {
        let data = json!({
            "aid": post_id,
            "prev":0,
        });

        let req: Value = self
            .client
            .post(COMMENT_URI)
            .header(
                REFERER,
                format!("https://time.geekbang.org/column/article/{}", post_id),
            )
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        println!("{}", req.to_string());
        Ok(())
    }
}

fn is_success_code(rsp_data: &Value) -> bool {
    if rsp_data["code"] == json!(0) {
        return true;
    }
    false
}
