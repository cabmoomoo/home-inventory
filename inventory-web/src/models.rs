use chrono::{DateTime, Local, Utc};
use log::Level;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub category: String,
    pub stock: isize,
    pub desired_stock: isize,
    pub track_general: bool,
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
            \"track_general\": {},
            \"last_updated\": \"{}\"
        ", self.id, self.name, self.category, self.stock, self.desired_stock, self.track_general, self.last_updated.to_string());
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

// #[derive(Serialize, Deserialize)]
// pub enum LogLevel {
//     Error,
//     Warn,
//     Info,
//     Debug,
//     Trace,
// }

#[derive(Deserialize)]
pub struct LogItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "log_date_format")]
    pub date: Option<DateTime<Local>>,
    pub level: Level,
    pub label: String,
    pub message: String
}

mod log_date_format {
    use chrono::{DateTime, Local};
    use serde::{Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %:z";

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<Local>>, D::Error> 
    where
        D: Deserializer<'de>    
    {
        let s = String::deserialize(deserializer)?;
        let dt: DateTime<Local> = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?.into();
        Ok(Some(dt))
    }
}