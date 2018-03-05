use models::db::DB;

use utils::config::Config;
use utils::invite_code::InviteCode;

use std::env;
pub struct InvitationService {
    client: Box<DB>,
    config: Config,
}

impl InvitationService {
    pub fn new() -> Self {
        let config = Config::new();
        let redis_host = env::var("REDIS_HOST").unwrap_or("".to_string());
        let redis_port = env::var("REDIS_PORT").unwrap_or("".to_string());
        let redis_password = env::var("REDIS_PASSWORD").unwrap_or("".to_string());
        let client = Box::new(DB::new(&redis_host, &redis_port, &redis_password).unwrap());
        Self { client, config }
    }
    pub fn query_user_invite_code(&self, user_id: &str) -> String {
        let client = &self.client;
        let mut invite_code = client.get_invite_code_by_user_id(user_id);
        if invite_code.is_empty() {
            let invite_code_id = client.get_incr_id();
            invite_code = InviteCode::new(invite_code_id);
            client.set_invite_code(user_id, &invite_code);
        }
        return invite_code;
    }
    pub fn query_user_invited_users(&self, user_id: &str) -> Vec<String> {
        let client = &self.client;
        client.get_success_invite_users(user_id)
    }
    pub fn add_user_invited_phone(&self, phone: &str, invite_code: &str) -> bool {
        let client = &self.client;
        client.set_phone_invite_code(phone, invite_code);
        let ok = true;
        return ok;
    }
    pub fn add_register_invited_user(&self, invite_user_id: &str, invited_phone: &str) -> (bool, String) {
        let (add_success, exists, missing_code, false_user_id) = (true, false, false, "".to_string());
        let client = &self.client;
        let invite_code = client.get_invite_code_by_phone(invited_phone);
        if invite_code.is_empty() {
            return (missing_code, false_user_id);
        }
        let user_id = client.get_user_id_by_invite_code(&invite_code);
        let invited_count = client.count_success_invite(&user_id);
        let (reach_limit, ok) = (false, true);
        if invited_count >= (&self.config).invited_limit {
            return (reach_limit, user_id);
        }
        if client.check_success_invite(&user_id, invite_user_id) {
            return (exists, user_id);
        }
        client.add_success_invite(&user_id, invite_user_id);
        return (add_success, user_id);
    }
}
