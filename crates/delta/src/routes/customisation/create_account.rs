use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;
use url::Url;
use std::error::Error;
use crate::util::regex::ADMIN_URL;

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
pub async fn create_account(
    authifier: &State<Authifier>,
    data: Json<DataCreateAccount>,
    mut shield: ShieldValidationInput,
) -> Result<EmptyResponse> {
    let data = data.into_inner();

    // post 请求要创建client
    let client = reqwest::Client::new();

    let url = ADMIN_URL + "/sso/registerWithoutAuthCode";

    let params = [("password", data.password),
        ("emailAddress", data.email)];

    // 发起post请求并返回
    Ok(client.post(url).form(&params).send().await?.json::<HashMap<String, Value>>().await?);

    Ok(EmptyResponse)
}