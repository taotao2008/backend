use crate::util::regex::RE_USERNAME;
use revolt_quark::{
    authifier::models::Session, models::User, Database, EmptyResponse, Error, Result,
};

use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use validator::Validate;

// 多元世界常量定义
pub static DEFAULT_SERVER_ID_1: &'static str = "01GVT45JCEKSAE6YRKA9NQWF3S";

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

    let server = db.fetch_server(&DEFAULT_SERVER_ID_1.to_owned()).await?;

    let user = db.fetch_user(&session.user_id.clone()).await?;

    //自动加入内置联邦
    server
        .create_member(db, user, None)
        .await
        .map(|_| EmptyResponse)
}
