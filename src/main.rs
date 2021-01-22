#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket_contrib::json::{Json, JsonValue};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/test")]
fn test() -> JsonValue  {
    let all_json = json!([
        {
            "name": "Vincent"
        },
        {
            "name": "Not Vincent"
        },
        {
            "name": "Not again"
        }
    ]);
    return all_json;
}


fn main() {
    rocket::ignite().mount("/", routes![hello, test]).launch();
}
