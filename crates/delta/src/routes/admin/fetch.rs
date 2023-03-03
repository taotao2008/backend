use async_recursion::async_recursion;
use revolt_quark::authifier::models::{Account, Session};
use revolt_quark::models::*;
use revolt_quark::{Db, Error, Result};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum FetchBy {
    WildcardId { id: String },
    AccountId { id: String },
    SessionId { id: String },
    SessionUserId { user_id: String },
    ChannelInviteId { id: String },
    ChannelId { id: String },
    MessageId { id: String },
    AttachmentId { id: String },
    EmojiId { id: String },
    ReportId { id: String },
    SnapshotId { id: String },
    ServerId { id: String },
    BotId { id: String },
    UserSettingsId { id: String },
    UserId { id: String },
}

impl FetchBy {
    #[async_recursion]
    pub async fn fetch(self, db: &Db) -> Result<Vec<Object>> {
        Ok(match self {
            FetchBy::WildcardId { id } => {
                vec![FetchBy::UserId { id: id.to_string() }.fetch(db).await]
                    .into_iter()
                    .filter_map(|res| res.ok())
                    .flatten()
                    .collect::<Vec<Object>>()
            }
            FetchBy::ChannelInviteId { id } => {
                vec![Object::ChannelInvite(db.fetch_invite(&id).await?)]
            }
            FetchBy::ChannelId { id } => {
                vec![Object::Channel(db.fetch_channel(&id).await?)]
            }
            FetchBy::MessageId { id } => {
                vec![Object::Message(db.fetch_message(&id).await?)]
            }
            FetchBy::AttachmentId { id } => todo!(),
            FetchBy::EmojiId { id } => {
                vec![Object::Emoji(db.fetch_emoji(&id).await?)]
            }
            FetchBy::ReportId { id } => vec![Object::Report(db.fetch_report(&id).await?)],
            _ => todo!(),
        })
    }
}

#[derive(Serialize, JsonSchema)]
#[serde(tag = "_type")]
pub enum Object {
    // Account(Account),
    ChannelInvite(Invite),
    ChannelUnread(ChannelUnread),
    Channel(Channel),
    Message(Message),
    Attachment(File),
    Emoji(Emoji),
    Report(Report),
    Session(Session),
    Snapshot(Snapshot),
    ServerBan(ServerBan),
    ServerMember(Member),
    Server(Server),
    Bot(Bot),
    UserSettings(UserSettings),
    User(User),
}

/// # Fetch Object
///
/// Fetch any object.
#[openapi(tag = "User Safety")]
#[post("/fetch", data = "<data>")]
pub async fn fetch(db: &Db, user: User, data: Json<FetchBy>) -> Result<Json<Vec<Object>>> {
    // Must be privileged for this route
    if !user.privileged {
        return Err(Error::NotPrivileged);
    }

    data.into_inner().fetch(db).await.map(Json)
}
