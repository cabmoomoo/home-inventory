#[macro_use]
extern crate rocket;

use std::{collections::HashMap, env, io::ErrorKind, sync::Arc};

use config::Config;
use cors::CORS;
use db::{AffectedRows, Item, DB};
use logging::*;
use rocket::{fs::{relative, FileServer, Options}, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{dbs::Session, kvs::Datastore};
use log::Level;
use log4rs;

mod db;
mod error;
mod prelude;
mod utils;
mod cors;
mod logging;

#[post("/item", format = "json", data = "<data>")]
async fn add_item(data: Json<Vec<String>>, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    let name = data[0].as_str();
    let category = data[1].as_str(); 
    let item = db
        .add_item(name, category)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create item."))?;

    log_database(LogItem { date: None, level: Level::Info, label: "Created new item:", message: &item.to_string() });

    Ok(Json(item))
}

#[post("/dev/item/<name>", format="json", data="<data>")]
async fn add_full_item(name: &str, data: Json<Vec<String>>, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    // [category, track_general, stock, desired_stock]
    let category = &data[0];
    let track_general = data[1].clone().parse().unwrap_or(false);
    let stock;
    let desired_stock;
    match data.get(2) {
        Some(s) => {
            stock = s.parse().unwrap_or_default();
            match data.get(3) {
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
        .add_full_item(name, category, stock, desired_stock, track_general)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create item."))?;

    log_database(LogItem { date: None, level: Level::Info, label: "Created new item:", message: &item.to_string() });

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

#[patch("/item/update/<id>", format="json", data="<data>", rank=1)]
async fn change_item(id: &str, data: Json<Item>, db: &State<DB>) -> Result<Json<Item>, std::io::Error> {
    let result = db
        .change_item(id, data.0)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    log_database(LogItem { date: None, level: Level::Info, label: "Changed item:", message: &result.to_string() });

    Ok(Json(result))
}

#[patch("/items/update", format="json", data="<data>")]
async fn change_items(data: Json<Vec<Item>>, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .change_items(data.0.clone())
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;
    
    log_database(LogItem { date: None, level: Level::Info, label: "Updated items:", message: &log_reinventory(data.0) });

    Ok(Json(result))
}

#[delete("/item/<id>")]
async fn delete_item(id: &str, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let result = db
        .delete_item(id)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    log_database(LogItem{date:None, level: Level::Warn, label: "Deleted: ", message: id });

    Ok(Json(result))
}

// #[post("/dev/dangerous", format="json", data="<data>")]
// async fn run_command(data: Json<&str>, db: &State<DB>) -> Result<Json<bool>, std::io::Error> {
//     let _result = db
//         .execute(data.0, None)
//         .await
//         .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

//     Ok(Json(true))
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestockItem {
    pub id: String,
    pub count: i64
} impl std::fmt::Display for RestockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.count)
    }
}

#[patch("/items/restock", format="json", data="<data>")]
async fn restock_items(data: Json<Vec<RestockItem>>, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let data = data.0;
    let result = db
        .restock_items(data.clone())
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    log_database(LogItem { date: None, level: Level::Info, label: "Restocked:", message: &log_vec(data) });

    Ok(Json(result))
}

#[patch("/items/consume", format="json", data="<data>")]
async fn consume_items(data: Json<Vec<RestockItem>>, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let data = data.0;
    let result = db
        .consume_items(data.clone())
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;

    log_database(LogItem { date: None, level: Level::Info, label: "Consumed:", message: &log_vec(data) });
    
    Ok(Json(result))
}

#[launch]
async fn rocket() -> _ {
    let mut config_init = Config::builder();
    config_init = match env::var("settings_path") {
        Ok(path) => config_init.add_source(config::File::with_name(&path)),
        Err(_) => config_init.add_source(config::File::with_name("inventory_config")),
    };
    let settings: HashMap<String, String> = config_init.build().unwrap().try_deserialize::<HashMap<String, String>>().unwrap();

    env::set_var("ROCKET_ADDRESS", settings.get("address").unwrap_or(&"127.0.0.1".to_owned()));
    env::set_var("ROCKET_PORT", settings.get("port").unwrap_or(&"26530".to_owned()));

    log4rs::init_config(logging::build_log_config(&settings)).unwrap();

    let ds = Arc::new(Datastore::new(settings.get("storage_path").unwrap_or(&"file://inventory.db".to_owned())).await.unwrap());
    let mut sesh = Session::default();

    sesh.ns = Some("my_ns".to_owned());
    sesh.db = Some("my_bd".to_owned());

    let db = DB {ds, sesh};

    rocket::build()
        .mount(
            "/",
            routes![add_item,
                add_full_item, 
                get_item, get_all_items, 
                restock_items, consume_items,
                change_item, change_items,
                delete_item,
                // run_command,
            ],
        )
        .mount(
            "/",
            FileServer::from("web")
        )
        .mount(
            "/logs",
            FileServer::new(settings.get("log_inventory_path").unwrap(), Options::None).rank(11)
        )
        .attach(CORS)
        .manage(db)
}