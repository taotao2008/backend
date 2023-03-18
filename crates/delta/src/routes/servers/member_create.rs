use revolt_quark::{
    models::{server_member::RemovalIntention, server_member::MemberCompositeKey, Member, User},
    perms, Db, EmptyResponse, Error, Permission, Ref, Result, Timestamp,
};



/// # Create Member
///
/// Removes a member from the server.
#[openapi(tag = "Server Members")]
#[post("/<target>/members/<member>")]
pub async fn req(db: &Db, user: User, target: Ref, member: Ref) -> Result<EmptyResponse> {
    let server = target.as_server(db).await?;

    if member.id == user.id {
        return Err(Error::CannotRemoveYourself);
    }

    if member.id == server.owner {
        return Err(Error::InvalidOperation);
    }



//    let member = member.as_member(db, &server.id).await?;
//
//
//    let member_create = Member {
//        id: MemberCompositeKey {
//            server: server.id.clone(),
//            user: member.id.clone(),
//        },
//        joined_at: Timestamp::now_utc(),
//        nickname: None,
//        avatar: None,
//        roles: vec![],
//        timeout: None,
//    };


    server
        .create_member(db, user, None)
        .await
        .map(|_| EmptyResponse)

}
