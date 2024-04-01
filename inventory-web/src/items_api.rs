use reqwasm::{http::Request, Error};

use crate::models::*;

const BASE_URL: &str = "http://192.168.1.11:8080";

pub async fn fetch_items() -> Result<Vec<Item>, Error> {
    Request::get(&format!("{BASE_URL}/items"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn new_item(name: &str, category: &str) -> Result<Item, Error> {
    Request::post(&format!("{BASE_URL}/item"))
        .body(format!("[\"{name}\", \"{category}\"]"))
        // .body(vec![name, category])
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn add_full_item(name: &str, category: &str, stock: i64, desired_stock: i64) -> Result<Item, Error> {
    Request::post(&format!("{BASE_URL}/dev/item/{name}"))
        .body(format!("[\"{category}\", \"{stock}\", \"{desired_stock}\"]"))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn change_item(id: &str, item: Item) -> Result<Item, Error> {
    Request::patch(&format!("{BASE_URL}/item/update/{id}"))
        .body(item.to_json())
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn delete_item(id: &str) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{BASE_URL}/item/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn restock_items(items: Vec<RestockItem>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/items/restock"))
        .body(restock_items_to_json(items))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn consume_items(items: Vec<RestockItem>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/items/consume"))
        .body(restock_items_to_json(items))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

// pub async fn test_request() -> Result<String, Error> {
//     Request::get(&format!("{BASE_URL}/items"))
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
// }

fn restock_items_to_json(items: Vec<RestockItem>) -> String {
    let mut result = "[".to_owned();
    for item in items {
        result += "{";
        result += &format!("\"id\": \"{}\",\"count\": {}", item.id, item.count);
        result += "},";
    }
    let _ = result.pop();
    result += "]";
    result
}