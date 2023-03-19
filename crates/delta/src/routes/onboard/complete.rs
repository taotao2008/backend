use crate::util::regex::RE_USERNAME;
use revolt_quark::{
    authifier::models::Session, models::User, models::Channel, Database, EmptyResponse, Error, Result,
};

use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use validator::Validate;
use ulid::Ulid;

use crate::util::const_def::DEFAULT_SERVER_ID_1;
use crate::util::const_def::DEFAULT_BOT_ID_1;

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



    //taotao 自动添加内置机器人1
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

    //taotao 创建内置DM

    // Otherwise try to find or create a DM.
    if let Ok(channel) = db.find_direct_message_channel(&session.user_id.clone(), &DEFAULT_BOT_ID_1.to_owned()).await {
        Json(channel);
    } else {
        let new_channel = Channel::DirectMessage {
            id: Ulid::new().to_string(),
            active: true,
            recipients: vec![session.user_id.clone(), DEFAULT_BOT_ID_1.to_owned()],
            last_message_id: None,
        };

        new_channel.create(db).await?;
        Json(new_channel);
    }


    let server = db.fetch_server(&DEFAULT_SERVER_ID_1.to_owned()).await?;
    //taotao 自动加入内置联邦
    server
        .create_member(db, user, None)
        .await
        .map(|_| EmptyResponse)



}
