use std::collections::BTreeMap;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{models::Item, InvCont};

pub enum DevTabMsg {
    AddItem,
    AddFullItem,
    ChangeItem,
    DeleteItem
}

#[derive(Properties, PartialEq)]
pub struct DevTabProps {
    pub controller: InvCont
}

pub struct DevTab {
    input_nodes: BTreeMap<String, NodeRef>
}

impl Component for DevTab {
    type Message = DevTabMsg;

    type Properties = DevTabProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut input_nodes = BTreeMap::new();
        let attrs = vec!["name", "category", "stock", "desired stock"];
        for attr in attrs {
            input_nodes.insert(attr.into(), NodeRef::default());
        }
        Self { input_nodes }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        let cont = &props.controller;
        let inventory = &props.controller.state.inventory;

        let clear_inputs;
        match msg {
            DevTabMsg::AddItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let category = self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().value();
                if name.is_empty() || category.is_empty() {
                    return false;
                }
                cont.new_item(name, category);
                clear_inputs = true;
            },
            DevTabMsg::AddFullItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let category = self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().value();
                let stock = self.input_nodes["stock"].cast::<HtmlInputElement>().unwrap().value().parse().unwrap_or(0);
                let desired_stock = self.input_nodes["desired stock"].cast::<HtmlInputElement>().unwrap().value().parse().unwrap_or(0);
                if name.is_empty() || category.is_empty() {
                    return false;
                }
                cont.add_full_item(name, category, stock, desired_stock);
                clear_inputs = true;
            },
            DevTabMsg::ChangeItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let category = self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().value();
                let stock = self.input_nodes["stock"].cast::<HtmlInputElement>().unwrap().value().parse().unwrap_or(0);
                let desired_stock = self.input_nodes["desired stock"].cast::<HtmlInputElement>().unwrap().value().parse().unwrap_or(0);
                if name.is_empty() || category.is_empty() {
                    return false;
                }
                let item_id;
                let id_fetch = &inventory.name_to_id.get(&AttrValue::from(name.clone()));
                match id_fetch {
                    Some(s) => item_id = *s,
                    None => return false,
                }
                let original = &inventory.item_id_map[item_id];
                let item = Item {
                    id: item_id.clone().to_string(),
                    name,
                    category,
                    stock,
                    desired_stock,
                    last_updated: original.last_updated.clone(),
                };
                cont.change_item(item_id.to_string(), item);
                clear_inputs = true;
            },
            DevTabMsg::DeleteItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let id_fetch = inventory.name_to_id.get(&AttrValue::from(name.clone()));
                let id;
                match id_fetch {
                    None => return false,
                    Some(i) => id = i.clone()
                }
                cont.delete_item(id.to_string());
                clear_inputs = true;
            },
        }
        if clear_inputs {
            for (_, node) in &self.input_nodes {
                node.cast::<HtmlInputElement>().unwrap().set_value("");
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html!(<div id="dev-tab">
        <div class="container"><table>
            <tr>
                <th rowspan="4"><div style="display: grid;">
                    <label>{"Name:"}</label>
                    <input type="text" ref={&self.input_nodes["name"]}/>
                    <label>{"Category:"}</label>
                    <input type="text" ref={&self.input_nodes["category"]}/>
                    <label>{"Stock:"}</label>
                    <input type="text" ref={&self.input_nodes["stock"]}/>
                    <label>{"Desired stock:"}</label>
                    <input type="text" ref={&self.input_nodes["desired stock"]}/>
                </div></th>
                <td>
                    <button onclick={ctx.link().callback(|_| DevTabMsg::AddItem)}>{"Add Item"}</button>
                </td>
            </tr>
            <tr>
                <td>
                    <button onclick={ctx.link().callback(|_| DevTabMsg::AddFullItem)}>{"Add Full Item"}</button>
                </td>
            </tr>
            <tr>
                <td>
                    <button onclick={ctx.link().callback(|_| DevTabMsg::ChangeItem)}>{"Change Item"}</button>
                </td>
            </tr>
            <tr>
                <td>
                <button onclick={ctx.link().callback(|_| DevTabMsg::DeleteItem)}>{"Delete Item"}</button>
                </td>
            </tr>
        </table></div>
        </div>)
    }
}