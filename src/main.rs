#![allow(dead_code, unused_variables, unused_imports)]

extern crate env_logger;
#[macro_use]
extern crate log;

extern crate serde;

#[macro_use]
extern crate serde_json;

extern crate redis;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod models;
use models::db::DB;

mod utils;

mod services;
use services::invitation::InvitationService;

mod views;

extern crate nickel;
use nickel::{HttpRouter, MiddlewareResult, Nickel, Options, Router};

extern crate clap;

extern crate num_cpus;

extern crate nickel_cors;

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut server = Nickel::new();

    let mut router = Nickel::router();
    router = views::urls::make_invitation_urls(router);

    server.utilize(views::api::InvitationAPI::default_media_type);
    server.utilize(nickel_cors::enable_cors);
    server.utilize(router);

    let server_ip = env::var("SERVER_IP").unwrap_or("127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or("6767".to_string());
    let workers = env::var("WORKERS").unwrap_or((num_cpus::get() * 2).to_string());
    info!(
        "server_ip={}, server_port={}, workers={}",
        server_ip, server_port, workers
    );
    server.options = Options::default().thread_count(Some(workers.parse::<usize>().unwrap_or(8)));

    server.listen(&format!("{}:{}", server_ip, server_port)).unwrap();
}
