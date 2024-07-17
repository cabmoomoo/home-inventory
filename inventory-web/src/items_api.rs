use reqwasm::{http::Request, Error};

use crate::models::*;

pub async fn fetch_items() -> Result<Vec<Item>, Error> {
    Request::get(&format!("/items"))
        .send()
        .await?
        .json()
        .await
}

pub async fn new_item(name: &str, category: &str) -> Result<Item, Error> {
    Request::post(&format!("/item"))
        .body(format!("[\"{name}\", \"{category}\"]"))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn add_full_item(name: &str, category: &str, stock: i64, desired_stock: i64, track_generally: bool) -> Result<Item, Error> {
    Request::post(&format!("/dev/item/{name}"))
        .body(format!("[\"{category}\", \"{track_generally}\", \"{stock}\", \"{desired_stock}\"]"))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn change_item(id: &str, item: Item) -> Result<Item, Error> {
    Request::patch(&format!("/item/update/{id}"))
        .body(item.to_json())
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn change_items(items: Vec<Item>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("/items/update"))
        .body(serde_json::to_string(&items).unwrap())
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn delete_item(id: &str) -> Result<AffectedRows, Error> {
    Request::delete(&format!("/item/{id}"))
        .send()
        .await?
        .json()
        .await
}

pub async fn restock_items(items: Vec<RestockItem>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("/items/restock"))
        .body(restock_items_to_json(items))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn consume_items(items: Vec<RestockItem>) -> Result<AffectedRows, Error> {
    Request::patch(&format!("/items/consume"))
        .body(restock_items_to_json(items))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await
}

pub async fn fetch_logs() -> Result<String, Error> {
    let running = Request::get("/logs/running.log")
        .send()
        .await?
        .text()
        .await;
    let old1 = Request::get("/logs/old1.log")
        .send()
        .await?;
    if !old1.ok() {
        return running;
    } else {
        return Ok(running.unwrap() + &old1.text().await.unwrap());
    }
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