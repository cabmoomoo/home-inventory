use chrono::Local;
use yew::prelude::*;

use crate::{items_api, models::LogItem, InvCont};

// #[derive(Deserialize)]
// pub struct LogItem {
//     pub date: DateTime<Local>,
//     pub level: String,
//     pub message: String
// }

pub enum LogTabMsg {
    LoadLogs(String)
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

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");

        match message {
            LogTabMsg::LoadLogs(current) => {
                let mut logs = parse_json_log(current, controller.clone());
                logs.sort_by(|a, b| b.date.unwrap().cmp(&a.date.unwrap()));

                let todays_date = Local::now().date_naive();
                let yesterdays_date = todays_date.checked_sub_days(chrono::Days::new(1)).unwrap();
                let mut current = vec![];
                let mut yesterday = vec![];

                for item in logs {
                    let item_date = item.date.unwrap().date_naive();
                    if item_date.lt(&yesterdays_date) {
                        break;
                    }
                    if item_date.eq(&todays_date) {
                        current.push(item);
                    } else if item_date.eq(&yesterdays_date) {
                        yesterday.push(item);
                    }
                }

                self.current = current;
                self.yesterday = yesterday;
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let mut current_logs: Vec<Html> = vec![];
        for log_item in self.current.iter() {
            current_logs.push(html!(<tr>
                <td class="date">{log_item.date.clone().unwrap().format("%r").to_string()}</td>
                <td>{log_item.label.clone() + &log_item.message}</td>
            </tr>))
        }
        let mut yesterday_logs: Vec<Html> = vec![];
        for log_item in self.yesterday.iter() {
            yesterday_logs.push(html!(<tr>
                <td class="date">{log_item.date.clone().unwrap().format("%r").to_string()}</td>
                <td>{log_item.label.clone() + &log_item.message}</td>
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

fn parse_json_log(log: String, inv_cont: InvCont) -> Vec<LogItem> {
    if log.is_empty() {
        return vec![];
    }

    let id_map = &inv_cont.state.inventory.item_id_map;

    let mut log = log;
    log.insert(0, '['); // Start the list
    log.pop(); // Pop newline
    log.pop(); // Pop trailing comma
    log.push(']'); // End the list
    let mut log_items: Vec<LogItem> = serde_json::from_str(&log).unwrap();

    for item in log_items.iter_mut() {
        if item.label.contains("Consumed") || item.label.contains("Restocked") || item.label.contains("Updated") {
            loop {
                let i = item.message.find("items:");
                match i {
                    None => break,
                    Some(i) => {
                        let mut item_id: String = item.message.drain(i..i+26).collect();
                        let inv_item_fetch = id_map.get(&AttrValue::from(item_id.clone()));
                        match inv_item_fetch {
                            Some(inv_item) => item.message.insert_str(i, &inv_item.name),
                            None => {
                                item_id.replace_range(4..5, "_id");
                                item.message.insert_str(i, &item_id);
                            },
                        }
                    }
                }
            }
        }
    }
    log_items
}