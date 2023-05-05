//! Create a new account
//! POST /account/create
use authifier::config::ShieldValidationInput;
use authifier::models::Account;
use authifier::{Authifier, Error, Result};
use rocket::serde::json::Json;
use rocket::State;
use rocket_empty::EmptyResponse;



//use authifier::models::Session;
//use authifier::{Authifier, Error, Result};
//use rocket::serde::json::Json;
//use rocket::State;
use std::collections::HashMap;
use serde_json::value::Value;

// 多元世界常量定义
pub static ADMIN_URL: &'static str = "https://bk-api.aizen.chat";

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

    // Check Captcha token
    authifier.config.captcha.check(data.captcha).await?;

    // Validate the request
    shield.email = Some(data.email.to_string());
    authifier.config.shield.validate(shield).await?;

    // Make sure email is valid and not blocked
    authifier
        .config
        .email_block_list;

    // Ensure password is safe to use
    authifier
        .config
        .password_scanning
        .assert_safe(&data.password)
        .await?;

    // If required, fetch valid invite
    let invite = if authifier.config.invite_only {
        if let Some(invite) = data.invite {
            Some(authifier.database.find_invite(&invite).await?)
        } else {
            return Err(Error::MissingInvite);
        }
    } else {
        None
    };


    let email = &data.email;
    let password = &data.password;

    // Create account
    let account = Account::new(authifier, email.to_string(), password.to_string(), true).await?;

    // Use up the invite
    if let Some(mut invite) = invite {
        invite.claimed_by = Some(account.id);
        invite.used = true;

        authifier.database.save_invite(&invite).await?;
    }

    //同步创建后台账号
    //if let Ok(res) = create_account_external(password.to_owned(), email.to_owned()).await {
      //  println!("{:#?}", res);
        //println!("{:#?}", res["message"]);
    //}


    Ok(EmptyResponse)
}


async fn create_account_external(password: String, email: String) -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    //let url = "https://bk-api.aizen.chat/sso/registerWithoutAuthCode";
    let url = ADMIN_URL.to_owned() + "/sso/registerWithoutAuthCode";


    let params = [("password", password),
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
        let (client, receiver) = bootstrap_rocket(
            "create_account",
            "success",
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "valid password"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let event = receiver.try_recv().expect("an event");
        if !matches!(event, AuthifierEvent::CreateAccount { .. }) {
            panic!("Received incorrect event type. {:?}", event);
        }
    }

    #[async_std::test]
    async fn fail_invalid_email() {
        let (client, _) = bootstrap_rocket(
            "create_account",
            "fail_invalid_email",
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "invalid",
                    "password": "valid password"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await,
            Some("{\"type\":\"IncorrectData\",\"with\":\"email\"}".into())
        );
    }

    #[async_std::test]
    async fn fail_invalid_password() {
        let (client, _) = bootstrap_rocket(
            "create_account",
            "fail_invalid_password",
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "password"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await,
            Some("{\"type\":\"CompromisedPassword\"}".into())
        );
    }

    #[async_std::test]
    async fn fail_invalid_invite() {
        let config = Config {
            invite_only: true,
            ..Default::default()
        };

        let (authifier, _) =
            for_test_with_config("create_account::fail_invalid_invite", config).await;
        let client = bootstrap_rocket_with_auth(
            authifier,
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "valid password",
                    "invite": "invalid"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await,
            Some("{\"type\":\"InvalidInvite\"}".into())
        );
    }

    #[async_std::test]
    async fn success_valid_invite() {
        let config = Config {
            invite_only: true,
            ..Default::default()
        };

        let (authifier, _) =
            for_test_with_config("create_account::success_valid_invite", config).await;
        let client = bootstrap_rocket_with_auth(
            authifier.clone(),
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let invite = Invite {
            id: "invite".to_string(),
            used: false,
            claimed_by: None,
        };

        authifier.database.save_invite(&invite).await.unwrap();

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "valid password",
                    "invite": "invite"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let invite = authifier
            .database
            .find_invite("invite")
            .await
            .expect("`Invite`");

        assert!(invite.used);
    }

    #[async_std::test]
    async fn fail_missing_captcha() {
        let config = Config {
            captcha: Captcha::HCaptcha {
                secret: "0x0000000000000000000000000000000000000000".into(),
            },
            ..Default::default()
        };

        let (authifier, _) =
            for_test_with_config("create_account::fail_missing_captcha", config).await;
        let client = bootstrap_rocket_with_auth(
            authifier,
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "valid password",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await,
            Some("{\"type\":\"CaptchaFailed\"}".into())
        );
    }

    #[async_std::test]
    async fn fail_captcha_invalid() {
        let config = Config {
            captcha: Captcha::HCaptcha {
                secret: "0x0000000000000000000000000000000000000000".into(),
            },
            ..Default::default()
        };

        let (authifier, _) =
            for_test_with_config("create_account::fail_invalid_captcha", config).await;
        let client = bootstrap_rocket_with_auth(
            authifier,
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "valid password",
                    "captcha": "00000000-aaaa-bbbb-cccc-000000000000"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await,
            Some("{\"type\":\"CaptchaFailed\"}".into())
        );
    }

    #[async_std::test]
    async fn success_captcha_valid() {
        let config = Config {
            captcha: Captcha::HCaptcha {
                secret: "0x0000000000000000000000000000000000000000".into(),
            },
            ..Default::default()
        };

        let (authifier, _) = for_test_with_config("create_account::success_captcha", config).await;
        let client = bootstrap_rocket_with_auth(
            authifier,
            routes![crate::routes::account::create_account::create_account],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "example@validemail.com",
                    "password": "valid password",
                    "captcha": "20000000-aaaa-bbbb-cccc-000000000002"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);
    }

    #[async_std::test]
    async fn success_smtp_sent() {
        let (authifier, _) = for_test_with_config(
            "create_account::success_smtp_sent",
            test_smtp_config().await,
        )
        .await;
        let client = bootstrap_rocket_with_auth(
            authifier,
            routes![
                crate::routes::account::create_account::create_account,
                crate::routes::account::verify_email::verify_email
            ],
        )
        .await;

        let res = client
            .post("/create")
            .header(ContentType::JSON)
            .body(
                json!({
                    "email": "create_account@smtp.test",
                    "password": "valid password",
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let mail = assert_email_sendria("create_account@smtp.test".into()).await;
        let res = client
            .post(format!("/verify/{}", mail.code.expect("`code`")))
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Ok);
    }
}
