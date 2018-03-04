pub struct Config {
    pub invited_limit: usize,
}

const INVITED_LIMIT: usize = 10;
use std::env;

impl Config {
    pub fn new() -> Self {
        let invited_limit = env::var("INVITED_LIMIT")
            .unwrap_or(INVITED_LIMIT.to_string())
            .parse::<usize>()
            .unwrap_or(INVITED_LIMIT);
        debug!("invited_limit={}", invited_limit);
        Self { invited_limit }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, INVITED_LIMIT};

    #[test]
    fn new_config() {
        let config = Config::new();
        assert_eq!(INVITED_LIMIT, config.invited_limit);
    }
}
