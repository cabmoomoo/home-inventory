use reqwasm::{http::Request, Error};

use crate::models::*;

const BASE_URL: &str = "http://192.168.1.229:26530";
// const BASE_URL: &str = "http://192.168.1.11:26530";
// const BASE_URL: &str = "http://127.0.0.1:26530";

pub async fn fetch_items() -> Result<Vec<Item>, Error> {
    Request::get(&format!("{BASE_URL}/items"))
        .send()
        .await?
        .json()
        .await
}

pub async fn new_item(name: &str, category: &str) -> Result<Item, Error> {
    Request::post(&format!("{BASE_URL}/item"))
        .body(format!("[\"{name}\", \"{category}\"]"))
        // .body(vec![name, category])
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn add_full_item(name: &str, category: &str, stock: i64, desired_stock: i64, track_generally: bool) -> Result<Item, Error> {
    Request::post(&format!("{BASE_URL}/dev/item/{name}"))
        .body(format!("[\"{category}\", \"{track_generally}\", \"{stock}\", \"{desired_stock}\"]"))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn change_item(id: &str, item: Item) -> Result<Item, Error> {
    Request::patch(&format!("{BASE_URL}/item/update/{id}"))
        .body(item.to_json())
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn change_items(items: Vec<Item>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/items/update"))
        .body(serde_json::to_string(&items).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn delete_item(id: &str) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{BASE_URL}/item/{id}"))
        .send()
        .await?
        .json()
        .await
}

pub async fn restock_items(items: Vec<RestockItem>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/items/restock"))
        .body(restock_items_to_json(items))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn consume_items(items: Vec<RestockItem>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("{BASE_URL}/items/consume"))
        .body(restock_items_to_json(items))
        .header("Content-Type", "application/json")
        .send()
        .await?
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

pub async fn fetch_logs() -> Result<(String, String), Error> {
    Request::get(&format!("{BASE_URL}/logs"))
        .send()
        .await?
        .json()
        .await
}

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