//
// database model
// authored by d-exclaimation
//
use serde::{Serialize, Deserialize};
use rocket_contrib::json::{JsonValue};

#[derive(Serialize, Deserialize)]
pub struct WishItemDTO {
    pub name: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WishItem {
    pub id: i64,
    pub name: String,
    pub price: i32,
}

impl WishItem {
    pub fn to_json(&self) -> JsonValue {
        json!({
            "id": self.id,
            "name": self.name,
            "price": self.price
        })
    }
}