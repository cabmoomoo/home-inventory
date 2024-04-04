use std::collections::BTreeMap;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{error_message, models::Item, InvCont, ItemSearch};

pub enum DevTabMsg {
    AddItem,
    AddFullItem,
    ChangeItem,
    DeleteItem,
    SearchedItem(AttrValue)
}

pub struct DevTab {
    input_nodes: BTreeMap<String, NodeRef>
}

impl Component for DevTab {
    type Message = DevTabMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut input_nodes = BTreeMap::new();
        let attrs = vec!["name", "category", "stock", "desired stock", "ID"];
        for attr in attrs {
            input_nodes.insert(attr.into(), NodeRef::default());
        }
        Self { input_nodes }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;
        let message = controller.message.clone();

        let clear_inputs;
        match msg {
            DevTabMsg::AddItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let category = self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().value();
                if name.is_empty() {
                    return false;
                }
                if category.is_empty() {
                    message.dispatch(error_message("A category is required for adding an item".into()));
                    return false;
                }
                if inventory.name_to_id.contains_key(&AttrValue::from(name.clone())) {
                    message.dispatch(error_message("An item with that name already exists".into()));
                    return false;
                }
                controller.new_item(name, category);
                clear_inputs = true;
            },
            DevTabMsg::AddFullItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let category = self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().value();
                let stock = self.input_nodes["stock"].cast::<HtmlInputElement>().unwrap().value().parse().unwrap_or(0);
                let desired_stock = self.input_nodes["desired stock"].cast::<HtmlInputElement>().unwrap().value().parse().unwrap_or(0);
                if name.is_empty() {
                    return false;
                }
                if category.is_empty() {
                    message.dispatch(error_message("A category is required for adding an item".into()));
                    return false;
                }
                if inventory.name_to_id.contains_key(&AttrValue::from(name.clone())) {
                    message.dispatch(error_message("An item with that name already exists".into()));
                    return false;
                }
                controller.add_full_item(name, category, stock, desired_stock);
                clear_inputs = true;
            },
            DevTabMsg::ChangeItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let category = self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().value();
                let stock_input = self.input_nodes["stock"].cast::<HtmlInputElement>().unwrap().value();
                let desired_stock_input = self.input_nodes["desired stock"].cast::<HtmlInputElement>().unwrap().value();
                let id = self.input_nodes["ID"].cast::<HtmlInputElement>().unwrap().value();

                if name.is_empty() && id.is_empty() {
                    return false;
                }
                if category.is_empty() {
                    message.dispatch(error_message("A category is required for all items".into()));
                    return false;
                }

                let item_id;
                let id_fetch = &inventory.item_id_map.get(&AttrValue::from(id.clone()));
                match id_fetch {
                    Some(_) => item_id = AttrValue::from(id.clone()),
                    None => {
                        let id_fetch_from_name = &inventory.name_to_id.get(&AttrValue::from(name.clone()));
                        match id_fetch_from_name {
                            Some(s) => item_id = (*s).clone(),
                            None => {
                                message.dispatch(error_message(format!("Could not find an item with name {}", name)));
                                return false;
                            },
                        }
                    }
                }

                let original = &inventory.item_id_map[&item_id];
                let stock = if stock_input.is_empty() {
                    original.stock.clone()
                } else {
                    match stock_input.parse() {
                        Ok(parsed) => parsed,
                        Err(_) => original.stock.clone()
                    }
                };
                let desired_stock = if desired_stock_input.is_empty() {
                    original.desired_stock.clone()
                } else {
                    match desired_stock_input.parse() {
                        Ok(parsed) => parsed,
                        Err(_) => original.desired_stock.clone()
                    }
                };

                let item = Item {
                    id: item_id.clone().to_string(),
                    name,
                    category,
                    stock,
                    desired_stock,
                    last_updated: original.last_updated.clone(),
                };

                controller.change_item(item_id.to_string(), item);
                clear_inputs = true;
            },
            DevTabMsg::DeleteItem => {
                let name = self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().value();
                let id = self.input_nodes["ID"].cast::<HtmlInputElement>().unwrap().value();
                let item_id;
                let id_fetch = &inventory.item_id_map.get(&AttrValue::from(id.clone()));
                match id_fetch {
                    Some(_) => item_id = AttrValue::from(id.clone()),
                    None => {
                        let id_fetch_from_name = &inventory.name_to_id.get(&AttrValue::from(name.clone()));
                        match id_fetch_from_name {
                            Some(s) => item_id = (*s).clone(),
                            None => {
                                message.dispatch(error_message(format!("Could not find an item with name {}", name)));
                                return false;
                            },
                        }
                    }
                }
                controller.delete_item(item_id.to_string());
                clear_inputs = true;
            },
            DevTabMsg::SearchedItem(item_id) => {
                let item = inventory.item_id_map.get(&item_id).unwrap();
                self.input_nodes["name"].cast::<HtmlInputElement>().unwrap().set_value(&item.name);
                self.input_nodes["category"].cast::<HtmlInputElement>().unwrap().set_value(&item.category);
                self.input_nodes["stock"].cast::<HtmlInputElement>().unwrap().set_value(&item.stock.to_string());
                self.input_nodes["desired stock"].cast::<HtmlInputElement>().unwrap().set_value(&item.desired_stock.to_string());
                self.input_nodes["ID"].cast::<HtmlInputElement>().unwrap().set_value(&item.id);
                clear_inputs = false;
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
        
        html!(<div id="dev-tab"><div class="container">
        <table>
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
                    <label>{"ID:"}</label>
                    <input type="text" ref={&self.input_nodes["ID"]}/>
                </div></th>
                <td>
                    <button onclick={ctx.link().callback(|_| DevTabMsg::AddItem)}>{"Add Item"}</button>
                </td>
            </tr>
            <tr><td>
                <button onclick={ctx.link().callback(|_| DevTabMsg::AddFullItem)}>{"Add Full Item"}</button>
            </td></tr>
            <tr><td>
                <button onclick={ctx.link().callback(|_| DevTabMsg::ChangeItem)}>{"Change Item"}</button>
            </td></tr>
            <tr><td>
                <button onclick={ctx.link().callback(|_| DevTabMsg::DeleteItem)}>{"Delete Item"}</button>
            </td></tr>
        </table>
        <ItemSearch selection_callback={ctx.link().callback(DevTabMsg::SearchedItem)}/>
        </div></div>)
    }
}