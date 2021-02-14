//
//  config.rust
//  rocket-kit
//
//  Created by d-exclaimation on 9:21 AM.
//  Copyright © 2020 d-exclaimation. All rights reserved.
//


use std::env;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

fn get_database() -> HashMap<&'static str, Value> {
    let db_url = get_db_url();
    let mut db_config = HashMap::new();
    db_config.insert("url", Value::from(db_url));
    let mut db = HashMap::new();
    db.insert("my_db", Value::from(db_config));
    return db;
}

fn get_db_url() -> String {
    env::var("DATABASE_URL")
        .ok()
        .unwrap_or("postgres://postgres@localhost/restful".to_string())
}

fn get_custom_db(database: String) -> String {
    env::var("DATABASE_URL")
        .ok()
        .unwrap_or(format!("postgres://postgres@localhost/{}", database))
}

fn get_server_port() -> u16 {
    env::var("ROCKET_PORT")
        .ok()
        .and_then(|res| res.parse().ok())
        .unwrap_or(8080)
}

fn get_environment() -> Environment {
    env::var("ROCKET_ENV")
        .ok()
        .and_then(|res| Option::from(if res == "production" {
            Environment::Production
        } else {
            Environment::Development
        }))
        .unwrap_or(Environment::Development)
}


pub fn setup_config() -> Config {
    Config::build(get_environment())
        .port(get_server_port())
        .extra("databases", get_database())
        .finalize()
        .unwrap()
}