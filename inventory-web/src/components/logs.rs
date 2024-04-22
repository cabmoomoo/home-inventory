use chrono::{DateTime, Datelike, Local};
use log::info;
use yew::prelude::*;

use crate::{items_api, InvCont};

pub struct LogItem {
    pub time: DateTime<Local>,
    pub level: AttrValue,
    pub msg: AttrValue
}

pub enum LogTabMsg {
    LoadLogs((String, String))
}

pub struct LogTab {
    pub current: Vec<LogItem>,
    pub yesterday: Vec<LogItem>
}

impl Component for LogTab {
    type Message = LogTabMsg;

    type Properties=();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { current: vec![], yesterday: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");

        match msg {
            LogTabMsg::LoadLogs((current, yesterday)) => {
                info!("{}, {}", current, yesterday);
                self.current = parse_log(current, controller.clone());
                self.current.sort_by(|a, b| b.time.cmp(&a.time));
                self.yesterday = parse_log(yesterday, controller.clone());
                self.yesterday.sort_by(|a, b| b.time.cmp(&a.time));
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let mut current_logs: Vec<Html> = vec![];
        for log_item in self.current.iter() {
            current_logs.push(html!(<tr>
                <td class="time">{log_item.time.format("%r").to_string()}</td>
                <td>{log_item.msg.clone()}</td>
            </tr>))
        }
        let mut yesterday_logs: Vec<Html> = vec![];
        for log_item in self.yesterday.iter() {
            yesterday_logs.push(html!(<tr>
                <td class="time">{log_item.time.format("%r").to_string()}</td>
                <td>{log_item.msg.clone()}</td>
            </tr>))
        }

        html!(<div id="logs-tab">
        <div class="container">
            <button class="log_reload" onclick={ctx.link().callback_future(|_| async {let logs = items_api::fetch_logs().await.unwrap_or_default(); LogTabMsg::LoadLogs(logs)})}>{"Reload Logs"}</button>
            <h3>{"Today's logs:"}</h3>
            <div class="log-container"><table>
                {for current_logs}
            </table></div>
            <h3>{"Yesterday's logs:"}</h3>
            <div class="log-container"><table>
                {for yesterday_logs}
            </table></div>
        </div>
        </div>)
    }
}

fn parse_log(log: String, inv_cont: InvCont) -> Vec<LogItem> {
    if log.is_empty() {
        return vec![];
    }
    let year = Local::now().year().to_string();
    let mut log_items: Vec<LogItem> = vec![];
    let search_value = "\n".to_owned() + &year;
    let mut log = log;

    loop {
        let i = log.rfind(&search_value);
        match i {
            None => break,
            Some(i) => {
                let item = log.split_off(i+1);
                log_items.push(parse_log_line(item, inv_cont.clone()));
                log.pop();
            }
        }
    }
    log_items.push(parse_log_line(log, inv_cont));
    log_items
}

fn parse_log_line(item: String, inv_cont: InvCont) -> LogItem {
    let time_zone = Local::now().offset().to_string();
    let id_map = &inv_cont.state.inventory.item_id_map;

    let mut item_iter = item.split(" | ").into_iter();
    let time = DateTime::parse_from_str(&(item_iter.next().unwrap().to_owned() + " " + &time_zone), "%Y-%m-%d %H:%M:%S %z").unwrap().into();
    let level = AttrValue::from(item_iter.next().unwrap().to_string());
    let mut msg = item_iter.next().unwrap().to_string();
    if msg.contains("Consumed:") || msg.contains("Restocked") {
        loop {
            let i = msg.find("items:");
            match i {
                None => break,
                Some(i) => {
                    let item_id: String = msg.drain(i..i+26).collect();
                    let item = id_map.get(&AttrValue::from(item_id)).unwrap();
                    msg.insert_str(i, &item.name);
                }
            }
        }
    }
    LogItem { time, level, msg: AttrValue::from(msg) }
}