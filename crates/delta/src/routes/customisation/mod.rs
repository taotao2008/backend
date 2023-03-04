use revolt_rocket_okapi::revolt_okapi::openapi3::OpenApi;
use rocket::Route;


mod emoji_create;
mod emoji_delete;
mod emoji_fetch;
mod create_account_custom;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        emoji_create::create_emoji,
        emoji_delete::delete_emoji,
        emoji_fetch::fetch_emoji,
        create_account_custom::create_account_custom
    ]
}
