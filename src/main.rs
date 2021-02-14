//
// main
// authored by d-exclaimation
//
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
mod db;
mod model;
mod config;
use rocket_contrib::json::{Json, JsonValue};
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

mod sql {
    // database object
    use rocket_contrib::databases::postgres;
    #[database("my_db")]
    pub struct MyPgDatabase(postgres::Connection);
}

// Handlers
#[get("/wishlist/<index>")]
pub fn get_handler(index: i64, conn: sql::MyPgDatabase) -> JsonValue {
    let data = db::get_one(index, &*conn);
    return json!(data)
}

#[get("/wishlist")]
pub fn get_all_handler(conn: sql::MyPgDatabase) -> JsonValue {
    let data_array = db::get_all(&*conn);
    return json!(data_array)
}

#[post("/wishlist", data = "<new_item>")]
pub fn post_handler(new_item: Json<model::WishItemDTO>, conn: sql::MyPgDatabase) -> JsonValue {
    let item = new_item.into_inner();
    let data = db::create_new(&item, &*conn);
    return json!(data)
}

#[put("/wishlist/<index>", data = "<updated_item>")]
pub fn put_handler(index: i64, updated_item: Json<model::WishItem>, conn: sql::MyPgDatabase) -> JsonValue {
    let item = updated_item.into_inner();
    if item.id != index {
        return json!({
            "status": "invalid id"
        })
    }
    let data = db::update(&item, &*conn);
    return json!(data)
}

#[delete("/wishlist/<index>")]
pub fn delete_handler(index: i64, conn: sql::MyPgDatabase) -> JsonValue {
    let stats = db::delete(index, &*conn);
    return json!({
        "status": if stats { "OK" } else { "invalid id" }
    })
}


fn main() {
    // Launch rocket server
    rocket::custom(config::setup_config())
        .attach(sql::MyPgDatabase::fairing())
        .mount("/", routes![get_handler, get_all_handler, post_handler, put_handler, delete_handler])
        .launch();
}
