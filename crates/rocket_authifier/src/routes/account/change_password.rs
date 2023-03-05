//! Change account password.
//! PATCH /account/change/password
use authifier::models::Account;
use authifier::util::hash_password;
use authifier::{Authifier, Result};
use rocket::serde::json::Json;
use rocket::State;
use rocket_empty::EmptyResponse;




use std::collections::HashMap;
use serde_json::value::Value;

pub static ADMIN_URL: &'static str = "http://bk.securechat.cn:8085";

/// # Change Data
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataChangePassword {
    /// New password
    pub password: String,
    /// Current password
    pub current_password: String,
}

/// # Change Password
///
/// Change the current account password.
#[openapi(tag = "Account")]
#[patch("/change/password", data = "<data>")]
pub async fn change_password(
    authifier: &State<Authifier>,
    mut account: Account,
    data: Json<DataChangePassword>,
) -> Result<EmptyResponse> {
    let data = data.into_inner();

    // Verify password can be used
    authifier
        .config
        .password_scanning
        .assert_safe(&data.password)
        .await?;

    // Ensure given password is correct
    account.verify_password(&data.current_password)?;



    let email = &account.email;
    let password = &data.password;
    let current_password = &data.current_password;


    //println!("{:#?}", email.to_owned());

    // Hash and replace password
    //account.password = hash_password(data.password)?;
    account.password = hash_password(password.to_string())?;


    if let Ok(res) = change_password_external(current_password.to_owned(), password.to_owned(), email.to_owned()).await {
       println!("{:#?}", res);
       //println!("{:#?}", res["message"]);
        if res["code"] != 200 {
            return Ok(EmptyResponse)
        }
   }

    // Commit to database
    account.save(authifier).await.map(|_| EmptyResponse)



}



async fn change_password_external(current_password: String, password: String, email: String) -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    let url = ADMIN_URL.to_owned() + "/sso/updatePasswordWithoutAuthCodeForMutil";


    let params = [("currentPassword", current_password),
        ("newPassword", password),
        ("emailAddress", email)];

    // 发起post请求并返回
    Ok(client.post(url).form(&params).send().await?.json::<HashMap<String, Value>>().await?)
}

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {
    use crate::test::*;

    #[async_std::test]
    async fn success() {
        use rocket::http::Header;

        let (authifier, session, _, _) = for_test_authenticated("change_password::success").await;
        let client = bootstrap_rocket_with_auth(
            authifier,
            routes![crate::routes::account::change_password::change_password],
        )
        .await;

        let res = client
            .patch("/change/password")
            .header(ContentType::JSON)
            .header(Header::new("X-Session-Token", session.token.clone()))
            .body(
                json!({
                    "password": "new password",
                    "current_password": "password_insecure"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let res = client
            .patch("/change/password")
            .header(ContentType::JSON)
            .header(Header::new("X-Session-Token", session.token))
            .body(
                json!({
                    "password": "sussy password",
                    "current_password": "new password"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);
    }
}
