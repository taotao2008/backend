use crate::util::regex::RE_USERNAME;
use revolt_quark::{
    authifier::models::Session, models::User, models::UserSettings, models::Channel, Database, EmptyResponse, Error, Result,
};


use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use validator::Validate;
use ulid::Ulid;
use std::collections::HashMap;
use revolt_quark::r#impl::UserSettingsImpl;
use chrono::Utc;

use crate::util::const_def::DEFAULT_SERVER_ID_1;
use crate::util::const_def::DEFAULT_BOT_ID_1;
use crate::util::const_def::DEFAULT_BOT_ID_2;

type Data = HashMap<String, String>;

/// # New User Data
#[derive(Validate, Serialize, Deserialize, JsonSchema)]
pub struct DataOnboard {
    /// New username which will be used to identify the user on the platform
    #[validate(length(min = 2, max = 32), regex = "RE_USERNAME")]
    username: String,
}


/// # Complete Onboarding
///
/// This sets a new username, completes onboarding and allows a user to start using Revolt.
#[openapi(tag = "Onboarding")]
#[post("/complete", data = "<data>")]
pub async fn req(
    db: &State<Database>,
    session: Session,
    user: Option<User>,
    data: Json<DataOnboard>,
) -> Result<EmptyResponse> {
    if user.is_some() {
        return Err(Error::AlreadyOnboarded);
    }

    let data = data.into_inner();
    data.validate()
        .map_err(|error| Error::FailedValidation { error })?;

    let username = User::validate_username(db, data.username).await?;
    let user = User {
        id: session.user_id.clone(),
        username,
        ..Default::default()
    };

    db.insert_user(&user).await.map(|_| EmptyResponse);



    //taotao 自动添加内置机器人1-chatgpt
    let mut user_request_1 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_1 = db.fetch_user(&DEFAULT_BOT_ID_1.to_owned()).await?;
    //taotao 步骤一：机器人发出添加好友请求
    user_bot_1.add_friend(db, &mut user_request_1).await?;
    Json(user_request_1.with_auto_perspective(db, &user_bot_1).await);

    //taotao 步骤二：好友同意机器人
    let mut user_accept_1 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_accept_1 = db.fetch_user(&DEFAULT_BOT_ID_1.to_owned()).await?;
    user_accept_1.add_friend(db, &mut user_bot_accept_1).await?;
    Json(user_bot_accept_1.with_auto_perspective(db, &user_accept_1).await);
    //taotao 自动添加内置机器人1-chatgpt-结束



    //taotao 自动添加内置机器人2-Dalle-E
    let mut user_request_2 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_2 = db.fetch_user(&DEFAULT_BOT_ID_2.to_owned()).await?;
    //taotao 步骤一：机器人发出添加好友请求
    user_bot_2.add_friend(db, &mut user_request_2).await?;
    Json(user_request_2.with_auto_perspective(db, &user_bot_2).await);

    //taotao 步骤二：好友同意机器人
    let mut user_accept_2 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_accept_2 = db.fetch_user(&DEFAULT_BOT_ID_2.to_owned()).await?;
    user_accept_2.add_friend(db, &mut user_bot_accept_2).await?;
    Json(user_bot_accept_2.with_auto_perspective(db, &user_accept_2).await);
    //taotao 自动添加内置机器人2-Dalle-E-结束

    //taotao 创建内置DM-机器人1-chatgpt
    // Otherwise try to find or create a DM.
    if let Ok(channel_1) = db.find_direct_message_channel(&session.user_id.clone(), &DEFAULT_BOT_ID_1.to_owned()).await {
        Json(channel_1);
    } else {
        let new_channel_1 = Channel::DirectMessage {
            id: Ulid::new().to_string(),
            active: true,
            recipients: vec![session.user_id.clone(), DEFAULT_BOT_ID_1.to_owned()],
            last_message_id: None,
        };

        new_channel_1.create(db).await?;
        Json(new_channel_1);
    }
    //taotao 创建内置DM-机器人1-chatgpt-结束


    //taotao 创建内置DM-机器人2-dalle-e
    // Otherwise try to find or create a DM.
    if let Ok(channel_2) = db.find_direct_message_channel(&session.user_id.clone(), &DEFAULT_BOT_ID_2.to_owned()).await {
        Json(channel_2);
    } else {
        let new_channel_2 = Channel::DirectMessage {
            id: Ulid::new().to_string(),
            active: true,
            recipients: vec![session.user_id.clone(), DEFAULT_BOT_ID_2.to_owned()],
            last_message_id: None,
        };

        new_channel_2.create(db).await?;
        Json(new_channel_2);
    }
    //taotao 创建内置DM-机器人2-dalle-e-结束

    //进行默认用户设置
    let data_json = json!({
        "appearance": { "appearance:emoji":"mutant", "appearance:show_send_button":true }
    });

    info!("data_json=");
    info!("{}", data_json.to_string());


    let current_time = Utc::now().timestamp_millis();

    let mut data_item = (current_time, data_json.to_string());
    info!("data_item=");
    info!("{:?}, {:?}", data_item);


    let mut settings_data: UserSettings = HashMap::new();
    settings_data.insert(session.user_id.clone(),data_item );

    info!("settings_data=");
    info!("{:?}, {:?}", settings_data.get(session.user_id.clone()));


    db.set_user_settings(&session.user_id.clone(), &settings_data ).await?;
    //进行默认用户设置-结束


    //taotao 自动加入内置联邦
    let server = db.fetch_server(&DEFAULT_SERVER_ID_1.to_owned()).await?;
    server
        .create_member(db, user, None)
        .await
        .map(|_| EmptyResponse)



}
