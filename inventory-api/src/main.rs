#[macro_use]
extern crate rocket;

use std::{env, io::ErrorKind, sync::Arc};

use cors::CORS;
use db::{AffectedRows, Item, DB};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{dbs::Session, kvs::Datastore};

mod db;
mod error;
mod prelude;
mod utils;
mod cors;

#[post("/item", format = "json", data = "<data>")]
async fn add_item(data: Json<Vec<String>>, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    let name = data[0].as_str();
    let category = data[1].as_str(); 
    let item = db
        .add_item(name, category)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create item."))?;

    Ok(Json(item))
}

#[patch("/item/<id>/desired/<desired_stock>")]
async fn set_desired_stock(id: &str, desired_stock: i64, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .set_desired_stock(id, desired_stock)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create item."))?;

    Ok(Json(result))
}

#[post("/dev/item/<name>", format="json", data="<data>")]
async fn add_full_item(name: &str, data: Json<Vec<String>>, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    let category = &data[0];
    let stock;
    let desired_stock;
    match data.get(1) {
        Some(s) => {
            stock = s.parse().unwrap_or_default();
            match data.get(2) {
                Some(d) => desired_stock = d.parse().unwrap_or_default(),
                None => desired_stock = 0,
            }
        },
        None => {
            stock = 0;
            desired_stock = 0;
        }
    }  
    let item = db
        .add_full_item(name, category, stock, desired_stock)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create item."))?;

    Ok(Json(item))
}

#[get("/item/<id>")]
async fn get_item(id: &str, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    let item = db
        .get_item(id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch item."))?;

    Ok(Json(item))
}

#[get("/items")]
async fn get_all_items(db: &State<DB>) -> Result<Json<Vec<Item>>, std::io::Error> {
    let items = db
        .get_all_items()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("Unable to fetch all items\n{}",e.to_string())))?;

    Ok(Json(items))
}

#[patch("/item/<id>/<stock>")]
async fn restock_item(id: &str, stock: i64, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .restock_item(id, stock)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(result))
}

#[patch("/item/<id>/consume/<stock>")]
async fn consume_item(id: &str, stock: i64, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .consume_item(id, stock)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(result))
}

#[patch("/item/update/<id>", format="json", data="<data>", rank=1)]
async fn change_item(id: &str, data: Json<Item>, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    let result = db
        .change_item(id, data.0)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(result))
}

#[delete("/item/<id>")]
async fn delete_item(id: &str, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .delete_item(id)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(result))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestockItem {
    pub id: String,
    pub count: i64
}

#[patch("/items/restock", format="json", data="<data>")]
async fn restock_items(data: Json<Vec<RestockItem>>, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    // let mut data_array = vec![];
    // for item in data.0 {
    //     // let data_obj: BTreeMap<String, Value> = map!(
    //     //     "id".into() => item.id.into(),
    //     //     "count".into() => item.count.into()
    //     // );
    //     let data_obj: Vec<Value> = vec![Value::Strand(item.id.into()), Value::Number(item.count.into())];
    //     data_array.push(Value::from(data_obj));
    // }

    let result = db
        .restock_items(data.0)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(result))
}

#[patch("/items/consume", format="json", data="<data>")]
async fn consume_items(data: Json<Vec<RestockItem>>, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .consume_items(data.0)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(Json(result))
}

#[launch]
async fn rocket() -> _ {
    let ds = Arc::new(Datastore::new("file://inventory.db").await.unwrap());
    // let ds = Arc::new(Datastore::new("memory").await.unwrap());
    let mut sesh = Session::default();

    sesh.ns = Some("my_ns".to_owned());
    sesh.db = Some("my_bd".to_owned());

    let db = DB {ds, sesh};

    env::set_var("ROCKET_ADDRESS", "192.168.1.229");
    env::set_var("ROCKET_PORT", "26530");
    env::set_var("ROCKET_LOG_LEVEL", "off");

    rocket::build()
        .mount(
            "/",
            routes![add_item, set_desired_stock,
                add_full_item, 
                get_item, get_all_items, 
                restock_item, consume_item, 
                restock_items, consume_items,
                change_item, delete_item
            ],
        )
        .attach(CORS)
        .manage(db)
}