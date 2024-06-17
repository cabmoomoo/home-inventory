use std::collections::BTreeMap;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{components::item_search::ItemSearch, models::RestockItem, InvCont};

#[derive(Serialize,Deserialize,Clone)]
pub struct DinnerlistStorage {
    items: BTreeMap<String, String>
}

pub enum DinnerListMsg {
    RetrieveStorage,
    UpdateStorage(AttrValue),
    AddItem(AttrValue),
    Submit
}

pub struct Dinnerlist {
    list_items: Vec<AttrValue>,
    item_nodes: BTreeMap<AttrValue, NodeRef>,
    general_nodes: BTreeMap<AttrValue, NodeRef>,
    storage: BTreeMap<String, String>,
    init_callback: Vec<ContextHandle<InvCont>>
}

impl Component for Dinnerlist {
    type Message = DinnerListMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut storage = BTreeMap::new();
        let mut init_callback = vec![];
        let storage_grab = LocalStorage::get::<BTreeMap<String, String>>("dinner_list");
        if storage_grab.is_ok() {
            storage = storage_grab.unwrap();
            let (_, handle) = ctx.link().context::<InvCont>(ctx.link().callback(|_| DinnerListMsg::RetrieveStorage)).expect("no ctx found");
            init_callback.push(handle);
        } else {
            // Err result could be because key doesn't exist, or becuase the key is malformed and couldn't be read.
            // No harm in deleting in either case.
            LocalStorage::delete("dinner_list");
        }
        Self { 
            list_items: vec![], 
            item_nodes: BTreeMap::new(), 
            general_nodes: BTreeMap::new(), 
            storage, 
            init_callback 
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;

        match msg {
            DinnerListMsg::RetrieveStorage => {
                self.list_items = vec![];
                self.item_nodes = BTreeMap::new();
                self.general_nodes = BTreeMap::new();
                for (item_id, _value) in self.storage.iter() {
                    let item_id = AttrValue::from(item_id.clone());
                    let item_get = &inventory.item_id_map.get(&item_id);
                    let item = match item_get {
                        Some(x) => x,
                        None => continue,
                    };
                    if item.track_general {
                        if !self.list_items.contains(&item_id) {
                            self.list_items.push(item_id.clone());
                            self.general_nodes.insert(item_id, NodeRef::default());
                        }
                    } else {
                        if !self.list_items.contains(&item_id) {
                            self.list_items.push(item_id.clone());
                            self.item_nodes.insert(item_id, NodeRef::default());
                        }
                    }
                }
                self.init_callback = vec![];
            },
            DinnerListMsg::UpdateStorage(item_id) => {
                if self.item_nodes.contains_key(&item_id) {
                    let node = &self.item_nodes[&item_id];
                    let value = node.cast::<HtmlInputElement>().unwrap().value();
                    self.storage.insert(item_id.to_string(), value);
                    let _ = LocalStorage::set("dinner_list", self.storage.clone());
                } else if self.general_nodes.contains_key(&item_id) {
                    let node = &self.general_nodes[&item_id];
                    let value = node.cast::<HtmlSelectElement>().unwrap().selected_index().to_string();
                    self.storage.insert(item_id.to_string(), value);
                    let _ = LocalStorage::set("dinner_list", self.storage.clone());
                }
                return false;
            },
            DinnerListMsg::AddItem(item_id) => {
                let item = &inventory.item_id_map[&item_id];
                if item.track_general {
                    if !self.list_items.contains(&item_id) {
                        self.list_items.push(item_id.clone());
                        self.general_nodes.insert(item_id, NodeRef::default());
                    }
                } else {
                    if !self.list_items.contains(&item_id) {
                        self.list_items.push(item_id.clone());
                        self.item_nodes.insert(item_id, NodeRef::default());
                    }
                }
                self.storage.insert(item.id.clone(), "".to_string());
                let _ = LocalStorage::set("dinner_list", self.storage.clone());
            },
            DinnerListMsg::Submit => {
                if self.item_nodes.len() == 0 && self.general_nodes.len() == 0 {
                    return false;
                }
                let mut items = vec![];
                for (id, node) in self.item_nodes.iter() {
                    let value = node.cast::<HtmlInputElement>().unwrap().value();
                    let mut count = match value.is_empty() {
                        false => value.parse().unwrap_or(1),
                        true => 1
                    };
                    if count == 0 {
                        continue;
                    }
                    let item = &inventory.item_id_map[id];
                    if item.stock < count {
                        count = item.stock.clone();
                    }
                    items.push(RestockItem { id: id.to_string(), count: count.try_into().unwrap() });
                }
                for (id, node) in self.general_nodes.iter() {
                    let selected_general_count = node.cast::<HtmlSelectElement>().unwrap().selected_options().item(0).unwrap().id();
                    let count;
                    if selected_general_count.eq("none") {
                        continue;
                    } else if selected_general_count.eq("low") {
                        count = 1;
                    } else { // "out"
                        if inventory.item_id_map[id].stock == 2 {
                            count = 2;
                        } else {
                            count = 1;
                        }
                    }
                    items.push(RestockItem { id: id.to_string(), count });
                }
                self.list_items = vec![];
                self.item_nodes = BTreeMap::new();
                self.general_nodes = BTreeMap::new();
                self.storage = BTreeMap::new();
                LocalStorage::delete("dinner_list");
                if items.len() > 0 {
                    controller.consume_items(items);
                }
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;
        let id_map = &inventory.item_id_map;

        let mut item_list: Vec<Html> = vec![];
        for item_id in self.list_items.iter() {
            let item = &id_map[item_id];
            let item_name = &item.name;
            if item.track_general {
                let stored = if self.storage.contains_key(&item_id.to_string()) {
                    self.storage[&item_id.to_string()].clone()
                } else {
                    "-1".to_string()
                };
                let mut general_options: Vec<Html> = vec![html!(<option value="none" id="none" selected={stored.eq("0")}>{"No Change"}</option>)];
                if item.stock > 0 {
                    general_options.push(html!(<option value="out" id="out" selected={stored.eq("1")}>{"Out"}</option>));
                }
                if item.stock > 1 {
                    general_options.push(html!(<option value="low" id="low" selected={stored.eq("2")}>{"Low"}</option>));
                }
                let item_id = item_id.clone();
                item_list.push(html!(<tr key={item_id.to_string()}>
                    <td class="name">{item_name}</td>
                    <td class="stock">
                        <select name={item_id.to_string()} id={item_id.to_string()} ref={&self.general_nodes[&item_id]} onchange={ctx.link().callback(move |_| DinnerListMsg::UpdateStorage(item_id.clone()))}>
                            {for general_options}
                        </select>
                    </td>
                </tr>))
            } else {
                let value = match self.storage.contains_key(&item_id.to_string()) {
                    true => Some(self.storage[&item_id.to_string()].clone()),
                    false => None,
                };
                let item_id = item_id.clone();
                item_list.push(html!(<tr key={item_id.to_string()}>
                    <td class="name">{item_name}</td>
                    <td class="stock">
                        <input type="number" size="5" min="1" placeholder="1" ref={self.item_nodes.get(&item_id).unwrap()} value={value} onchange={ctx.link().callback(move |_| DinnerListMsg::UpdateStorage(item_id.clone()))} />
                    </td>
                </tr>));
            }
        }

        html!(<div id="dinner-list" class="item-stock-tab">
        <div class="container">
            <ItemSearch selection_callback={ctx.link().callback(DinnerListMsg::AddItem)}/>
            <table>
                {for item_list}
            </table>
            <button onclick={ctx.link().callback(|_| DinnerListMsg::Submit)}>{"Submit"}</button>
        </div>
        </div>)
    }
}