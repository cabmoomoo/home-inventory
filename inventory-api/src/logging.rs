use std::collections::HashMap;

use chrono::{DateTime, Local};
use log::{log, Level};
use log4rs::{
    append::{
        console::ConsoleAppender, 
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, 
            trigger::size::SizeTrigger, 
            CompoundPolicy
        }
    }, 
    config::{Appender, Logger, Root}, 
    encode::pattern::PatternEncoder, 
    Config
};
use serde::{Deserialize, Serialize};

use crate::db::Item;

// #[derive(Serialize, Deserialize)]
// pub enum LogLevel {
//     Error,
//     Warn,
//     Info,
//     Debug,
//     Trace,
// }

#[derive(Serialize, Deserialize)]
pub struct LogItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Local>>,
    pub level: Level,
    pub label: &'a str,
    pub message: &'a str
}

pub fn log_database(log_item: LogItem) {
    log!(target: "database", log_item.level, "\"label\": \"{}\", \"message\": \"{}\"", log_item.label, log_item.message);
}

pub fn log_vec(input: Vec<impl std::fmt::Display>) -> String {
    let mut result = String::default();
    for item in input {
        result += &format!("\\n\\t{}", item);
    }
    result
}

pub fn log_reinventory(input: Vec<Item>) -> String {
    let mut result = String::default();
    for item in input {
        result += &format!("\\n\\t{}: {}:{}", item.id.unwrap(), item.stock, item.desired_stock);
    }
    result
}

pub fn build_log_config(settings: &HashMap<String, String>) -> Config {
    // The console appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}")))
        .build();

    // File logger for all untargeted output (so mainly Rocket logs)
    let general_logger = {
        let path = settings.get("log_general_path").unwrap();
        let trigger = SizeTrigger::new(250000);
        let roller = FixedWindowRoller::builder()
            .base(1)
            .build(&(path.clone() + "/old{}.log"), 4)
            .unwrap();
        let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
        log4rs::append::rolling_file::RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {h({l})}: {m}{n}")))
            .build(&(path.clone() + "/running.log"), Box::new(policy))
            .unwrap()
    };

    // File logger for all database modifications (add, consume, restock items)
    let database_mods = {
        let path = settings.get("log_inventory_path").unwrap();
        let trigger = SizeTrigger::new(5000);
        let roller = FixedWindowRoller::builder()
            .base(1)
            .build(&(path.clone() + "/old{}.log"), 10)
            .unwrap();
        let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
        log4rs::append::rolling_file::RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{{\"date\": \"{d(%Y-%m-%d %H:%M:%S %Z)}\", \"level\": \"{l}\", {m}}},{n}")))
            .build(&(path.clone() + "/running.log"), Box::new(policy))
            .unwrap()
    };
    
    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("general_logger", Box::new(general_logger)))
        .appender(Appender::builder().build("database_mods", Box::new(database_mods)))
        .logger(Logger::builder()
            .appender("database_mods")
            .additive(false)
            .build("database", log::LevelFilter::Info)
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("general_logger")
                .build(log::LevelFilter::Warn)
        )
        .unwrap()
}