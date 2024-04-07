use std::collections::BTreeMap;

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{components::item_search::ItemSearch, models::RestockItem, InvCont};

pub enum DinnerListMsg {
    AddItem(AttrValue),
    Submit
}

pub struct Dinnerlist {
    list_items: Vec<AttrValue>,
    item_nodes: BTreeMap<AttrValue, NodeRef>,
    general_nodes: BTreeMap<AttrValue, NodeRef>
}

impl Component for Dinnerlist {
    type Message = DinnerListMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { list_items: vec![], item_nodes: BTreeMap::new(), general_nodes: BTreeMap::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;

        match msg {
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
                    if selected_general_count.eq("low") {
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
                controller.consume_items(items);
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
                let mut general_options: Vec<Html> = vec![html!(<option value="out" id="out">{"Out"}</option>)];
                if item.stock == 2 {
                    general_options.push(html!(<option value="low" id="low">{"Low"}</option>));
                    // general_options.reverse();
                }
                item_list.push(html!(<tr key={item_id.to_string()}>
                    <td class="name">{item_name}</td>
                    <td class="stock">
                        <select name={item_id.to_string()} id={item_id.to_string()} ref={&self.general_nodes[item_id]}>
                            {for general_options}
                        </select>
                    </td>
                </tr>))
            } else {
                item_list.push(html!(<tr key={item_id.to_string()}>
                    <td class="name">{item_name}</td>
                    <td class="stock">
                        <input type="number" size="5" min="1" placeholder="1" ref={self.item_nodes.get(item_id).unwrap()} />
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