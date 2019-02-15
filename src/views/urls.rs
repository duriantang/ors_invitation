extern crate nickel;

use nickel::{HttpRouter, Nickel, Router};

use crate::views::api::InvitationAPI;

pub fn make_invitation_urls(mut router: Router) -> Router {
    let url_prefix = "/api/invitation";

    router.get(
        url_prefix.to_owned() + "/query/user/invite_code/",
        InvitationAPI::query_user_invite_code_api,
    );

    router.get(
        url_prefix.to_owned() + "/query/user/invited_users/",
        InvitationAPI::query_user_invited_users_api,
    );

    router.get(
        url_prefix.to_owned() + "/add/invited_phone/",
        InvitationAPI::add_user_invited_phone_api,
    );

    router.get(
        url_prefix.to_owned() + "/add/register_invited_user/",
        InvitationAPI::add_register_invited_user_api,
    );
    router.get("/-/ping/", InvitationAPI::pong);
    router
}
