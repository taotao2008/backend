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
use crate::util::const_def::DEFAULT_SERVER_ID_2;
use crate::util::const_def::DEFAULT_BOT_ID_1;
use crate::util::const_def::DEFAULT_BOT_ID_2;
use crate::util::const_def::DEFAULT_BOT_ID_3;
use crate::util::const_def::DEFAULT_BOT_ID_4;
use crate::util::const_def::DEFAULT_HELP_ID_3;

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

    //taotao 自动添加内置机器人3-Midjouney
    let mut user_request_3 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_3 = db.fetch_user(&DEFAULT_BOT_ID_3.to_owned()).await?;
    //taotao 步骤一：机器人发出添加好友请求
    user_bot_3.add_friend(db, &mut user_request_3).await?;
    Json(user_request_3.with_auto_perspective(db, &user_bot_3).await);

    //taotao 步骤二：好友同意机器人
    let mut user_accept_3 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_accept_3 = db.fetch_user(&DEFAULT_BOT_ID_3.to_owned()).await?;
    user_accept_3.add_friend(db, &mut user_bot_accept_3).await?;
    Json(user_bot_accept_3.with_auto_perspective(db, &user_accept_3).await);
    //taotao 自动添加内置机器人3-Midjouney-结束


    //taotao 自动添加内置机器人4-Openjouney
    let mut user_request_4 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_4 = db.fetch_user(&DEFAULT_BOT_ID_4.to_owned()).await?;
    //taotao 步骤一：机器人发出添加好友请求
    user_bot_4.add_friend(db, &mut user_request_4).await?;
    Json(user_request_4.with_auto_perspective(db, &user_bot_4).await);

    //taotao 步骤二：好友同意机器人
    let mut user_accept_4 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_accept_4 = db.fetch_user(&DEFAULT_BOT_ID_4.to_owned()).await?;
    user_accept_4.add_friend(db, &mut user_bot_accept_4).await?;
    Json(user_bot_accept_4.with_auto_perspective(db, &user_accept_4).await);
    //taotao 自动添加内置机器人4-Openjouney-结束



    //taotao 自动添加内置AiZen客服为好友
    let mut user_request_3 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_3 = db.fetch_user(&DEFAULT_HELP_ID_3.to_owned()).await?;
    //taotao 步骤一：AiZen客服发出添加好友请求
    user_bot_3.add_friend(db, &mut user_request_3).await?;
    Json(user_request_3.with_auto_perspective(db, &user_bot_3).await);

    //taotao 步骤二：好友同意AiZen客服
    let mut user_accept_3 = db.fetch_user(&session.user_id.clone()).await?;
    let mut user_bot_accept_3 = db.fetch_user(&DEFAULT_HELP_ID_3.to_owned()).await?;
    user_accept_3.add_friend(db, &mut user_bot_accept_3).await?;
    Json(user_bot_accept_3.with_auto_perspective(db, &user_accept_3).await);
    //taotao 自动添加内置AiZen客服-结束



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


    //taotao 创建内置DM-机器人3-Midjourney
    // Otherwise try to find or create a DM.
    if let Ok(channel_3) = db.find_direct_message_channel(&session.user_id.clone(), &DEFAULT_BOT_ID_3.to_owned()).await {
        Json(channel_3);
    } else {
        let new_channel_3 = Channel::DirectMessage {
            id: Ulid::new().to_string(),
            active: true,
            recipients: vec![session.user_id.clone(), DEFAULT_BOT_ID_3.to_owned()],
            last_message_id: None,
        };

        new_channel_3.create(db).await?;
        Json(new_channel_3);
    }
    //taotao 创建内置DM-机器人3-Midjourney-结束

    //taotao 创建内置DM-机器人4-Openjourney
    // Otherwise try to find or create a DM.
    if let Ok(channel_4) = db.find_direct_message_channel(&session.user_id.clone(), &DEFAULT_BOT_ID_4.to_owned()).await {
        Json(channel_4);
    } else {
        let new_channel_4 = Channel::DirectMessage {
            id: Ulid::new().to_string(),
            active: true,
            recipients: vec![session.user_id.clone(), DEFAULT_BOT_ID_4.to_owned()],
            last_message_id: None,
        };

        new_channel_4.create(db).await?;
        Json(new_channel_4);
    }
    //taotao 创建内置DM-机器人4-Openjourney-结束



    //taotao 创建内置DM-AiZen客服
    // Otherwise try to find or create a DM.
    if let Ok(channel_3) = db.find_direct_message_channel(&session.user_id.clone(), &DEFAULT_HELP_ID_3.to_owned()).await {
        Json(channel_3);
    } else {
        let new_channel_3 = Channel::DirectMessage {
            id: Ulid::new().to_string(),
            active: true,
            recipients: vec![session.user_id.clone(), DEFAULT_HELP_ID_3.to_owned()],
            last_message_id: None,
        };

        new_channel_3.create(db).await?;
        Json(new_channel_3);
    }
    //taotao 创建内置DM-AiZen客服-结束

    //进行默认用户设置
    let current_time = Utc::now().timestamp_millis();
    //定义默认show button
    let data_show_button_json = json!({
        "appearance:emoji":"mutant",
        "appearance:show_send_button":true
    });
    let mut data_item_show_button = (current_time, data_show_button_json.to_string());

    //定义默认语言
    let data_lang_json = json!({"lang":"zh_Hans"});
    let mut data_item_lang = (current_time, data_lang_json.to_string());


    let mut settings_data: UserSettings = HashMap::new();
    //增加默认show button
    settings_data.insert("appearance".to_string(), data_item_show_button );
    //增加默认语言
    settings_data.insert("locale".to_string(), data_item_lang );

    db.set_user_settings(&session.user_id.clone(), &settings_data ).await?;
    //进行默认用户设置-结束


    //taotao 自动加入内置联邦1-OpenAI联邦
    let server_1 = db.fetch_server(&DEFAULT_SERVER_ID_1.to_owned()).await?;
    server_1
        .create_member(db, user, None)
        .await
        .map(|_| EmptyResponse);
    //taotao 自动加入内置联邦1-OpenAI联邦-end

    //taotao 自动加入内置联邦2-Midjourney联邦
    let mut user_2 = db.fetch_user(&session.user_id.clone()).await?;
    let server_2 = db.fetch_server(&DEFAULT_SERVER_ID_2.to_owned()).await?;
    server_2
        .create_member(db, user_2, None)
        .await
        .map(|_| EmptyResponse)
    //taotao 自动加入内置联邦2-Midjourney联邦-end


}
