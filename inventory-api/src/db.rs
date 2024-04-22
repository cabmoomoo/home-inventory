
use std::{collections::BTreeMap, fmt, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    dbs::{Response, Session}, kvs::Datastore, sql::{thing, Array, Object, Value}
};

use crate::{prelude::{Error, W}, utils::macros::map};

#[derive(Debug, Serialize, Deserialize)]
pub enum Categories {
    Can,
    Box,
    Bag,
    Frozen,
    Refridgerated,
    Other
}
impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub category: String,
    pub stock: i64,
    pub desired_stock: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_general: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>
} impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\tid: {}", &self.id.clone().unwrap_or("None".to_string()))?;
        writeln!(f, "\tname: {}", &self.name)?;
        writeln!(f, "\tcategory: {}", &self.category)?;
        writeln!(f, "\tstock {}", &self.stock)?;
        writeln!(f, "\tdesired_stock: {}", &self.desired_stock)?;
        writeln!(f, "\ttrack_general: {}", &self.track_general.unwrap_or(false))?;
        write!(f, "\tlast_updated: {}", &self.last_updated.unwrap())
    }
}

// impl From<W<Object>> for Item {
//     fn from(obj: W<Object>) -> Self {
//         let map = obj.0;
//         Self { id: Some(map["id"].to_string()), 
//             name: map["name"].to_string(), 
//             category: map["category"].to_string(), 
//             stock: map["stock"].clone().try_into().expect("value wasn't a number"), 
//             desired_stock: map["desired_stock"].clone().try_into().expect("value wasn't a number"), 
//             last_updated: Some(map["last_updated"].clone().try_into().expect("value wasn't a datetime"))
//         }
//     }
// }

impl TryFrom<W<Object>> for Item {
    type Error = Error;
    fn try_from(val: W<Object>) -> Result<Self, Error> {
        let map = val.0.clone();
        Ok(Self {
            id: Some(W(map["id"].clone()).try_into()?), 
            name: W(map["name"].clone()).try_into()?, 
            category: W(map["category"].clone()).try_into()?,
            stock: W(map["stock"].clone()).try_into()?,
            desired_stock: W(map["desired_stock"].clone()).try_into()?,
            track_general: Some(W(map["track_general"].clone()).try_into()?),
            last_updated: Some(W(map["last_updated"].clone()).try_into()?)
        })
    }
}
impl TryFrom<W<Value>> for Item {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Self, Self::Error> {
        match val.0 {
            Value::Object(obj) => W(obj).try_into(),
            _ => Err(Error::XValueNotOfType("object")),
        }
    }
}
 /* 
impl From<Item> for Value {
    fn from(val: Item) -> Self {
        map!(
            "id".into() => val.id.into(),
            "name".into() => val.name.into(),
            "category".into() => val.category.into(),
            "stock".into() => val.stock.into(),
            "desired_stock".into() => val.desired_stock.into(),
        )
        .into()
    }
}

impl Creatable for Item {}

pub trait Creatable: Into<Value> {}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: usize,
}

#[derive(Clone)]
pub struct DB {
    pub ds: Arc<Datastore>,
    pub sesh: Session,
}

impl DB {
    pub async fn execute(
        &self, 
        query: &str,
        vars: Option<BTreeMap<String, Value>>
    ) -> Result<Vec<Response>, crate::error::Error> {
        let res = self.ds.execute(query, &self.sesh, vars).await?;
        Ok(res)
    }
    
    pub async fn add_item(&self, name: &str, category: &str) -> Result<Item, crate::error::Error> {
        let sql = "CREATE items SET name = $name, category = $category, stock = 0, desired_stock = 0, track_general = false, last_updated = time::now()";
        let vars: BTreeMap<String, Value> = map!(
            "name".into() => Value::Strand(name.into()),
            "category".into() => Value::Strand(category.into())
        );
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn set_desired_stock(&self, id: &str, desired_stock: i64) -> Result<AffectedRows, crate::error::Error> {
        let sql = "UPDATE $th SET desired_stock = $desired_stock;";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!(
            "th".into() => thing(&tid)?.into(),
            "desired_stock".into() => Value::Number(desired_stock.into())
        );
        let _ = self.execute(sql, Some(vars)).await?;
        Ok(AffectedRows { rows_affected: 1 })
    }

    pub async fn add_full_item(&self, name: &str, category: &str, stock: i64, desired_stock: i64, track_general: bool) -> Result<Item, crate::error::Error> {
        let sql = "CREATE items SET name = $name, category = $category, stock = $stock, desired_stock = $desired_stock, track_general = $track_general, last_updated = time::now()";
        let vars: BTreeMap<String, Value> = map!(
            "name".into() => Value::Strand(name.into()),
            "category".into() => Value::Strand(category.into()),
            "stock".into() => Value::Number(stock.into()),
            "desired_stock".into() => Value::Number(desired_stock.into()),
            "track_general".into() => Value::Bool(track_general.into())
        );
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_item(&self, id: &str) -> Result<Item, crate::error::Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!("th".into() => thing(&tid)?.into());
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_all_items(&self) -> Result<Vec<Item>, crate::error::Error> {
        let sql = "SELECT * FROM items ORDER BY last_updated ASC;";

        let res = self.execute(sql, None).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()

        // let temp: Vec<_> = array.into_iter().map(|value| <W<surrealdb::sql::Value> as TryInto<Object>>::try_into(W(value))).collect::<Vec<_>>();

        // temp.into_iter().map(|value| W(value).try_into()).collect()
        // todo!()
    }

    pub async fn restock_item(&self, id: &str, stock: i64) -> Result<AffectedRows, crate::error::Error> {
        let sql = "UPDATE $th SET stock += $stock, last_updated = time::now();";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!(
            "th".into() => thing(&tid)?.into(),
            "stock".into() => Value::Number(stock.into())
        );
        let _ = self.execute(sql, Some(vars)).await?;
        Ok(AffectedRows { rows_affected: 1 })

        // let first_res = res.into_iter().next().expect("Did not get a response");

        // let array1: Array = W(first_res.result?).try_into()?;
        // let array2: Array = W(array1.0.into_iter().next().unwrap()).try_into()?;

        // array2.0.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn consume_item(&self, id: &str, stock: i64) -> Result<AffectedRows, crate::error::Error> {
        let sql = "UPDATE $th SET stock -= $stock, last_updated = time::now();";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!(
            "th".into() => thing(&tid)?.into(),
            "stock".into() => Value::Number(stock.into())
        );
        let _ = self.execute(sql, Some(vars)).await?;
        Ok(AffectedRows { rows_affected: 1 })
    }

    pub async fn restock_items(&self, data: Vec<crate::RestockItem>) -> Result<AffectedRows, crate::error::Error> {
        let mut sql = "BEGIN TRANSACTION;".to_owned();
        for item in data.iter() {
            sql += &format!("UPDATE {} SET stock += {}, last_updated = time::now();", item.id, item.count.to_string());
        }
        sql += "COMMIT TRANSACTION;";
        let _ = self.execute(&sql, None).await?;
        Ok(AffectedRows { rows_affected: data.len() })
    }

    pub async fn consume_items(&self, data: Vec<crate::RestockItem>) -> Result<AffectedRows, crate::error::Error> {
        let mut sql = "BEGIN TRANSACTION;".to_owned();
        for item in data.iter() {
            sql += &format!("UPDATE {} SET stock -= {}, last_updated = time::now();", item.id, item.count.to_string());
        }
        sql += "COMMIT TRANSACTION;";
        let _ = self.execute(&sql, None).await?;
        Ok(AffectedRows { rows_affected: data.len() })
    }

    pub async fn change_item(&self, id: &str, item: Item) -> Result<Item, crate::error::Error> {
        let sql = "UPDATE $th SET name = $name, category = $category, stock = $stock, desired_stock = $desired_stock, track_general = $track_general, last_updated = time::now()";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!(
            "th".into() => thing(&tid)?.into(),
            "name".into() => Value::Strand(item.name.into()),
            "category".into() => Value::Strand(item.category.into()),
            "stock".into() => Value::Number(item.stock.into()),
            "desired_stock".into() => Value::Number(item.desired_stock.into()),
            "track_general".into() => Value::Bool(item.track_general.unwrap_or(false).into())
        );
        let res = self.execute(sql, Some(vars)).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn delete_item(&self, id: &str) -> Result<AffectedRows, crate::error::Error> {
        let sql = "DELETE $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!(
            "th".into() => thing(&tid)?.into()
        );
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}