#[macro_use]
extern crate rocket;
#[macro_use]
extern crate revolt_rocket_okapi;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod routes;
pub mod util;

use async_std::channel::unbounded;
use revolt_quark::authifier::{Authifier, AuthifierEvent};
use revolt_quark::events::client::EventV1;
use revolt_quark::DatabaseInfo;

#[launch]
async fn rocket() -> _ {
    // Configure logging and environment
    revolt_quark::configure!();

    // Ensure environment variables are present
    revolt_quark::variables::delta::preflight_checks();

    // Setup database
    let db = DatabaseInfo::Auto.connect().await.unwrap();
    db.migrate_database().await.unwrap();

    // Setup Authifier event channel
    let (sender, receiver) = unbounded();

    // Setup Authifier
    let authifier = Authifier {
        database: db.clone().into(),
        config: revolt_quark::util::authifier::config(),
        event_channel: Some(sender),
    };

    // Launch a listener for Authifier events
    async_std::task::spawn(async move {
        while let Ok(event) = receiver.recv().await {
            match &event {
                AuthifierEvent::CreateSession { .. } | AuthifierEvent::CreateAccount { .. } => {
                    EventV1::Auth(event).global().await
                }
                AuthifierEvent::DeleteSession { user_id, .. }
                | AuthifierEvent::DeleteAllSessions { user_id, .. } => {
                    let id = user_id.to_string();
                    EventV1::Auth(event).private(id).await
                }
            }
        }
    });

    // Launch background task workers
    async_std::task::spawn(revolt_quark::tasks::start_workers(db.clone()));

    // Configure CORS
    let cors = revolt_quark::web::cors::new();

    // Configure Rocket
    let rocket = rocket::build();
    routes::mount(rocket)
        .mount("/", revolt_quark::web::cors::catch_all_options_routes())
        .mount("/", revolt_quark::web::ratelimiter::routes())
        .mount("/swagger/", revolt_quark::web::swagger::routes())
        //.mount("/auth/account", routes![create_account_custom])
        .manage(authifier)
        .manage(db)
        .manage(cors.clone())
        .attach(revolt_quark::web::ratelimiter::RatelimitFairing)
        .attach(cors)
}
/*

use revolt_quark::authifier::config::ShieldValidationInput;
use revolt_quark::authifier::models::Account;
use revolt_quark::authifier::{Error, Result};

use rocket::serde::json::Json;
use rocket::State;
use rocket_empty::EmptyResponse;
use serde::Deserialize;
use validator::Validate;
use serde::Serialize;

use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;
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
#[post("/create", data = "<data>", rank=0 )]
pub async fn create_account_custom(
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
        .email_block_list
        .validate_email(&data.email)?;

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
    if let Ok(res) = create_account(password.to_owned(), email.to_owned()).await {
        println!("{:#?}", res);
        //println!("{:#?}", res["message"]);
    }



    Ok(EmptyResponse)
}


async fn create_account(password: String, email: String) -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    //let url = "http://bk.securechat.cn:8085/sso/registerWithoutAuthCode";
    let url = ADMIN_URL.to_owned() + "/sso/registerWithoutAuthCode";


    let params = [("password", password),
        ("emailAddress", email)];

    // 发起post请求并返回
    Ok(client.post(url).form(&params).send().await?.json::<HashMap<String, Value>>().await?)
}*/
