use redis::{Commands, ConnectionAddr, ConnectionInfo, PipelineCommands};

const INVITE_CODE_CONTER_KEY: &str = "ORS_INVITATION:conter";
const INVITE_CODE_USER_KEY: &str = "ORS_INVITATION:invite_code_set";
const USER_INVITE_CODE_KEY: &str = "ORS_INVITATION:user_invite_code_set";
const PHONE_INVITE_CODE_KEY: &str = "ORS_INVITATION:phone_invite_code_set";
const PHONE_INVITE_USER_KEY: &str = "ORS_INVITATION:phone_invite_user_set";
const SUCCESS_INVITE_USER_SET_KEY_PREFIX: &str = "ORS_INVITATION:success_invite_user:";

pub struct DB {
    pub con: redis::Connection,
    invite_code_counter_key: &'static str,
    invite_code_user_key: &'static str,
    user_invite_code_key: &'static str,
    phone_invite_code_key: &'static str,
    phone_invite_user_key: &'static str,
    success_invite_user_set_key_prefix: &'static str,
}

impl DB {
    pub fn new(host: &str, port: &str, password: &str) -> Result<DB, redis::RedisError> {
        let db: i64 = 0;
        let password = match password.is_empty() {
            true => None,
            false => Some(password.to_string()),
        };
        let redis_connection_info = ConnectionInfo {
            addr: Box::new(ConnectionAddr::Tcp(
                host.to_string(),
                port.to_string().parse::<u16>().unwrap_or(6379),
            )),
            db: db,
            passwd: password,
        };
        let client = redis::Client::open(redis_connection_info)?;
        let con = client.get_connection()?;
        Ok(DB {
            con,
            invite_code_counter_key: INVITE_CODE_CONTER_KEY,
            invite_code_user_key: INVITE_CODE_USER_KEY,
            user_invite_code_key: USER_INVITE_CODE_KEY,
            phone_invite_code_key: PHONE_INVITE_CODE_KEY,
            phone_invite_user_key: PHONE_INVITE_USER_KEY,
            success_invite_user_set_key_prefix: SUCCESS_INVITE_USER_SET_KEY_PREFIX,
        })
    }

    pub fn set(&self, key: &str, val: &str) -> Result<(), redis::RedisError> {
        let _: () = self.con.set(key, val)?;
        Ok(())
    }

    pub fn invite_code_exists(&self, invite_code: &str) -> bool {
        self.con
            .hexists(self.invite_code_user_key, invite_code)
            .unwrap_or(false)
    }

    pub fn get_user_id_by_invite_code(&self, invite_code: &str) -> String {
        self.con
            .hget(self.invite_code_user_key, invite_code)
            .unwrap_or("".to_string())
    }

    pub fn get_invite_code_by_user_id(&self, user_id: &str) -> String {
        self.con
            .hget(self.user_invite_code_key, user_id)
            .unwrap_or("".to_string())
    }

    pub fn get_incr_id(&self) -> u64 {
        self.con.incr(self.invite_code_counter_key, "1").unwrap()
    }

    pub fn set_invite_code(&self, user_id: &str, invite_code: &str) -> bool {
        let exists = self.invite_code_exists(invite_code);
        println!("{:?}", exists);

        if exists {
            return false;
        }
        let keys = vec![self.invite_code_user_key, self.user_invite_code_key];
        let _result: Vec<u32> = redis::transaction(&self.con, &keys, |pipe| {
            pipe.hset(self.user_invite_code_key, user_id, invite_code)
                .ignore()
                .hset(self.invite_code_user_key, invite_code, user_id)
                .query(&self.con)
        })
        .unwrap();
        true
    }

    pub fn set_phone_invite_code(&self, phone: &str, invite_code: &str) -> String {
        self.con
            .hset(self.phone_invite_code_key, phone, invite_code)
            .unwrap_or("".to_string())
    }
    pub fn get_invite_code_by_phone(&self, phone: &str) -> String {
        self.con
            .hget(self.phone_invite_code_key, phone)
            .unwrap_or("".to_string())
    }

    pub fn set_phone_user(&self, phone: &str, invite_code: &str) -> String {
        let user_id = self.get_user_id_by_invite_code(invite_code);
        if user_id.is_empty() {
            return "".to_string();
        }
        self.con
            .hset(self.phone_invite_user_key, phone, user_id)
            .unwrap_or("".to_string())
    }

    pub fn check_success_invite(&self, user_id: &str, invite_user_id: &str) -> bool {
        let key = self.success_invite_user_set_key_prefix.to_string() + user_id;
        self.con.sismember(key, invite_user_id).unwrap_or(false)
    }

    pub fn add_success_invite(&self, user_id: &str, invite_user_id: &str) -> String {
        let key = self.success_invite_user_set_key_prefix.to_string() + user_id;
        self.con.sadd(key, invite_user_id).unwrap_or("".to_string())
    }

    pub fn count_success_invite(&self, user_id: &str) -> usize {
        let key = self.success_invite_user_set_key_prefix.to_string() + user_id;
        self.con.scard(key).unwrap_or(0usize)
    }
    pub fn get_success_invite_users(&self, user_id: &str) -> Vec<String> {
        let key = self.success_invite_user_set_key_prefix.to_string() + user_id;
        self.con.smembers(key).unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::{Commands, DB};
    use redis::ConnectionLike;

    fn setup() -> DB {
        DB::new("redis", "6379", "").unwrap()
    }

    #[test]
    fn test_new_db() {
        let db = setup();
        assert_eq!(0, db.con.get_db());
    }
}
