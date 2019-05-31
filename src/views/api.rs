use nickel::{MediaType, MiddlewareResult, QueryString, Request, Response};

use crate::services::invitation::InvitationService;

extern crate serde_json;

pub struct InvitationAPI {}

impl InvitationAPI {
    pub fn default_media_type<'mw>(
        _req: &mut Request,
        mut resp: Response<'mw>,
    ) -> MiddlewareResult<'mw> {
        resp.set(MediaType::Json);
        resp.next_middleware()
    }
    pub fn query_user_invite_code_api<'mw>(
        req: &mut Request,
        resp: Response<'mw>,
    ) -> MiddlewareResult<'mw> {
        let user_id = match req.query().all("user_id") {
            Some(l) => l[0].to_string(),
            None => "".to_string(),
        };
        debug!("{:?}", user_id);

        if user_id.is_empty() {
            let data = json!({
                "ok": false
            });
            return resp.send(data.to_string());
        };
        let service = InvitationService::new();
        let invite_code = service.query_user_invite_code(&user_id);
        let data = json!({
            "ok": true,
            "data": {
                "invite_code": invite_code,
                "user_id": user_id
            }
        });
        return resp.send(data.to_string());
    }

    pub fn query_user_invited_users_api<'mw>(
        req: &mut Request,
        resp: Response<'mw>,
    ) -> MiddlewareResult<'mw> {
        let user_id = match req.query().all("user_id") {
            Some(l) => l[0].to_string(),
            None => "".to_string(),
        };
        debug!("{:?}", user_id);

        if user_id.is_empty() {
            let data = json!({
                "ok": false
            });
            return resp.send(data.to_string());
        };
        let service = InvitationService::new();
        let users = service.query_user_invited_users(&user_id);
        let count = users.len();
        let data = json!({
            "ok": true,
            "data": {
                "users": users,
                "count": count
            }
        });
        return resp.send(data.to_string());
    }

    pub fn add_user_invited_phone_api<'mw>(
        req: &mut Request,
        resp: Response<'mw>,
    ) -> MiddlewareResult<'mw> {
        let phone = match req.query().all("phone") {
            Some(l) => l[0].to_string(),
            None => "".to_string(),
        };
        let invite_code = match req.query().all("invite_code") {
            Some(l) => l[0].to_string(),
            None => "".to_string(),
        };
        if phone.is_empty() || invite_code.is_empty() {
            let data = json!({
                "ok": false
            });
            return resp.send(data.to_string());
        }
        let service = InvitationService::new();
        let users = service.add_user_invited_phone(&phone, &invite_code);
        let data = json!({
            "ok": true
            }
        );
        return resp.send(data.to_string());
    }

    pub fn add_register_invited_user_api<'mw>(
        req: &mut Request,
        resp: Response<'mw>,
    ) -> MiddlewareResult<'mw> {
        let invite_user_id = match req.query().all("user_id") {
            Some(l) => l[0].to_string(),
            None => "".to_string(),
        };
        let invited_phone = match req.query().all("phone") {
            Some(l) => l[0].to_string(),
            None => "".to_string(),
        };
        if invite_user_id.is_empty() || invited_phone.is_empty() {
            let data = json!({
                "ok": false
            });
            return resp.send(data.to_string());
        }
        let service = InvitationService::new();
        let (result, user_id) = service.add_register_invited_user(&invite_user_id, &invited_phone);
        let data = json!({
            "ok": true,
            "data": {
                "user_id": user_id,
                "result": result
                }
            }
        );
        return resp.send(data.to_string());
    }

    pub fn pong<'mw>(req: &mut Request, mut resp: Response<'mw>) -> MiddlewareResult<'mw> {
        resp.set(MediaType::Txt);
        resp.send("pong")
    }
}
