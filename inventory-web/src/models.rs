use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub category: String,
    pub stock: isize,
    pub desired_stock: isize,
    pub last_updated: DateTime<Utc>
} impl Item {
    pub fn to_json(&self) -> String {
        let mut result = "{".to_owned();
        result += &format!("
            \"id\": \"{}\",
            \"name\": \"{}\",
            \"category\": \"{}\",
            \"stock\": {},
            \"desired_stock\": {},
            \"last_updated\": \"{}\"
        ", self.id, self.name, self.category, self.stock, self.desired_stock, self.last_updated.to_string());
        result += "}";
        result
    }
}

#[derive(Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64
}

#[derive(Serialize, Deserialize)]
pub struct RestockItem {
    pub id: String,
    pub count: i64
}