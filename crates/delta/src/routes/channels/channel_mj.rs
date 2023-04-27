use revolt_quark::{
    models::{channel::PartialChannel, Channel, User, Message},
    perms, Db, EmptyResponse, Error, Permission, Ref, Result,
    web::idempotency::IdempotencyKey,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use ulid::Ulid;

use std::collections::HashMap;
use serde_json::value::Value;

use crate::util::const_def::MIDJOURNEY_URL;

/// # Query Parameters
#[derive(Validate, Serialize, Deserialize, JsonSchema, FromForm)]
pub struct OptionsMidjourneyGet {

    index: Option<String>,
    message_id: Option<String>,
    image_hash: Option<String>,
}

/// # Channel U
///
/// get a server channel, leaves a group or closes a group.
#[openapi(tag = "Channel Information")]
#[get("/<target>?<options..>")]
pub async fn req(
    db: &Db,
    user: User,
    target: Ref,
    options: OptionsMidjourneyGet,
) -> Result<EmptyResponse> {



    let index = options.index.unwrap_or_default();
    let message_id = options.message_id.unwrap_or_default();
    let image_hash = options.image_hash.unwrap_or_default();

    let channel = target.as_channel(db).await?;

    // Start constructing the message
    let message_id = Ulid::new().to_string();
    let mut message = Message {
        id: message_id.clone(),
        channel: channel.id().to_string(),
        author: user.id.clone(),

        ..Default::default()
    };

    // 1. Parse mentions in message.


    // 2. Verify permissions for masquerade.


    // 3. Ensure interactions information is correct

    // 4. Verify replies are valid.


    // 5. Process included embeds.


    // 6. Add attachments to message.

    // 7. Set content
    let content = "/up_scale ".to_owned() + &index.to_owned() + &message_id.to_owned() + &image_hash.to_owned();

    let option_content: Option<String> = Some(content.to_owned());


    message.content = option_content;

    // 8. Pass-through nonce value for clients


    message.create(db, &channel, Some(&user)).await?;
    Ok(EmptyResponse)
}


