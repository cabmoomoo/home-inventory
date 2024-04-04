use std::collections::BTreeMap;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{models::RestockItem, InvCont, ItemSearch};

pub enum GroceryBagMsg {
    AddItem(AttrValue),
    Submit
}

pub struct GroceryBag {
    list_items: Vec<AttrValue>,
    item_nodes: BTreeMap<AttrValue, NodeRef>
}

impl Component for GroceryBag {
    type Message = GroceryBagMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { list_items: vec![], item_nodes: BTreeMap::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");

        match msg {
            GroceryBagMsg::AddItem(item_id) => {
                if !self.list_items.contains(&item_id) {
                    self.list_items.push(item_id.clone());
                    self.item_nodes.insert(item_id, NodeRef::default());
                }
            },
            GroceryBagMsg::Submit => {
                if self.item_nodes.len() == 0 {
                    return false;
                }
                let mut items = vec![];
                for (id, node) in self.item_nodes.iter() {
                    let value = node.cast::<HtmlInputElement>().unwrap().value();
                    let count = match value.is_empty() {
                        false => value.parse().unwrap_or(1),
                        true => 1
                    };
                    if count == 0 {
                        continue;
                    }
                    items.push(RestockItem { id: id.to_string(), count });
                }
                self.list_items = vec![];
                self.item_nodes = BTreeMap::new();
                controller.restock_items(items);
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
            let item_name = &id_map[item_id].name;
            item_list.push(html!(<tr key={item_id.to_string()}>
                <td class="name">{item_name}</td>
                <td class="stock">
                    <input type="number" size="5" min="1" placeholder="1" ref={self.item_nodes.get(item_id).unwrap()} />
                </td>
            </tr>));
        }

        html!(<div id="grocery-bag" class="item-stock-tab">
        <div class="container">
            <ItemSearch selection_callback={ctx.link().callback(GroceryBagMsg::AddItem)}/>
            <table>
                {for item_list}
            </table>
            <button onclick={ctx.link().callback(|_| GroceryBagMsg::Submit)}>{"Submit"}</button>
        </div>
        </div>)
    }
}