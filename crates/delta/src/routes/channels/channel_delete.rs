use revolt_quark::{
    models::{channel::PartialChannel, Channel, User},
    perms, Db, EmptyResponse, Error, Permission, Ref, Result,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// # Query Parameters
#[derive(Validate, Serialize, Deserialize, JsonSchema, FromForm)]
pub struct OptionsChannelDelete {
    /// Whether to not send a leave message
    leave_silently: Option<bool>,
}

/// # Close Channel
///
/// Deletes a server channel, leaves a group or closes a group.
#[openapi(tag = "Channel Information")]
#[delete("/<target>?<options..>")]
pub async fn req(
    db: &Db,
    user: User,
    target: Ref,
    options: OptionsChannelDelete,
) -> Result<EmptyResponse> {

    let mut channel = target.as_channel(db).await?;
    let mut perms = perms(&user).channel(&channel);
    perms.throw_permission(db, Permission::ViewChannel).await?;

    match &channel {
        Channel::SavedMessages { .. } => Err(Error::NoEffect),
        Channel::DirectMessage { .. } => {
            let mut target = target.as_user(db).await?;
            //taotao 内置机器人的DM不能删除
            if user.bot.is_some() || target.bot.is_some() {
                return Err(Error::IsBot);
            }
            channel
                .update(
                    db,
                    PartialChannel {
                        active: Some(false),
                        ..Default::default()
                    },
                    vec![],
                )
                .await
                .map(|_| EmptyResponse)
        },
        Channel::Group { .. } => channel
            .remove_user_from_group(
                db,
                &user.id,
                None,
                options.leave_silently.unwrap_or_default(),
            )
            .await
            .map(|_| EmptyResponse),
        Channel::TextChannel { .. } | Channel::VoiceChannel { .. } => {
            perms
                .throw_permission(db, Permission::ManageChannel)
                .await?;

            channel.delete(db).await.map(|_| EmptyResponse)
        }
    }
}
