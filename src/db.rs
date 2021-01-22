//
// database handler
// authored by d-exclaimation
//
use crate::model::{WishItem, WishItemDTO};
use rocket_contrib::json::{JsonValue};
use rocket_contrib::databases::postgres;
use rocket_contrib::databases::postgres::rows::Row;

pub fn get_all(conn: &postgres::Connection) -> Vec<JsonValue> {
    let res: Vec<JsonValue> = vec_jsons(&conn
        .query(
            "SELECT * FROM wishlist",
            &[]
        )
    );
    return res;
}

pub fn get_one(index: i64, conn: &postgres::Connection) -> JsonValue {
    let res: Vec<JsonValue> = vec_jsons(&conn
        .query(
            "SELECT * FROM wishlist WHERE list_id = $1",
            &[&index]
        )
    );
    return res[0].clone();
}

pub fn create_new(new_item: &WishItemDTO, conn: &postgres::Connection) -> JsonValue {
    let res : Vec<JsonValue> = vec_jsons(&conn
        .query(
            "INSERT INTO wishlist (name, price) VALUES ($1, $2) RETURNING *",
            &[&new_item.name, &new_item.price]
        )
    );
    return res[0].clone()
}

pub fn update(updated: &WishItem, conn: &postgres::Connection) -> JsonValue {
    let res: Vec<JsonValue> = vec_jsons(&conn
        .query(
            "UPDATE wishlist SET name = $1, price = $2 WHERE list_id = $3 RETURNING *",
            &[&updated.name, &updated.price, &updated.id]
        )
    );
    return res[0].clone();
}

pub fn delete(index: i64, conn: &postgres::Connection) -> bool {
    let count = conn.execute("DELETE FROM wishlist WHERE list_id = $1", &[&index]).unwrap();
    return count > 0
}

fn row_to_item(row: Row) -> JsonValue {
    let res = WishItem{
        id: row.get("list_id"),
        name: row.get("name"),
        price: row.get("price"),
    };
    return res.to_json()
}

pub fn vec_jsons(res: &postgres::Result<postgres::rows::Rows>) -> Vec<JsonValue> {
    res.as_ref().unwrap().iter().map(row_to_item).collect()
}

