
use serde::Deserialize;
use serde::Serialize;
use revolt_quark::authifier::{Result};
use rocket::serde::json::Json;

use rocket_empty::EmptyResponse;



/// # Account Data
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataCreateAccount {
    /// Valid email address
    pub email: String,
    /// Password
    pub password: String,
    /// Invite code
    pub invite: Option<String>,
    /// Captcha verification code
    pub captcha: Option<String>,
}

/// # Create Account
///
/// Create a new account.
#[openapi(tag = "Account")]
#[post("/create", data = "<data>")]
pub async fn create_account (
    data: Json<DataCreateAccount>,
) -> Result<EmptyResponse> {
    let data = data.into_inner();

    // post 请求要创建client
    let client = reqwest::Client::new();

    let url =  "http://bk.securechat.cn:8085/sso/registerWithoutAuthCode";

    let params = [("password", data.password),
        ("emailAddress", data.email)];

    // 发起post请求并返回
    client.post(url).form(&params).send().await;
    Ok(EmptyResponse)
}