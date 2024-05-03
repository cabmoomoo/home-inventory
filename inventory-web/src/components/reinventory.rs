use std::collections::{BTreeMap, HashMap};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{models::Item, InvCont};

pub enum ReInventoryMsg {
    Submit
}

pub struct ReInventory {
    inputs: HashMap<AttrValue, (NodeRef, NodeRef)>,
    //            HashMap<Category Name, Vec<Item ID>>
    category_map: BTreeMap<String, Vec<AttrValue>>
}

impl Component for ReInventory {
    type Message = ReInventoryMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;
        let mut inputs = HashMap::new();
        let mut category_map: BTreeMap<String, Vec<AttrValue>> = BTreeMap::new();
        for (id, item) in inventory.item_id_map.iter() {
            let cat_fetch = category_map.get_mut(&item.category);
            match cat_fetch {
                Some(cat) => {cat.push(id.clone());},
                None => {category_map.insert(item.category.clone(), vec![id.clone()]);},
            }
            inputs.insert(id.clone(), (NodeRef::default(), NodeRef::default()));
        }
        for (_cat, items) in category_map.iter_mut() {
            items.sort_unstable_by(|a, b| inventory.item_id_map[a].name.cmp(&inventory.item_id_map[b].name));
        }
        Self { inputs, category_map }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;

        let mut item_list: Vec<Item> = vec![];
        for (id, (stock_ref, desired_ref)) in self.inputs.iter() {
            let new_stock = stock_ref.cast::<HtmlInputElement>().unwrap().value();
            let new_desired = desired_ref.cast::<HtmlInputElement>().unwrap().value();
            if !new_stock.is_empty() || !new_desired.is_empty() {
                let mut item_changed = false;
                let mut new_item = inventory.item_id_map[id].clone();
                if !new_stock.is_empty() {
                    let stock = new_stock.parse().unwrap_or(new_item.stock);
                    if stock != new_item.stock {
                        new_item.stock = stock;
                        item_changed = true;
                    }
                }
                if !new_desired.is_empty() {
                    let desired = new_desired.parse().unwrap_or(new_item.desired_stock);
                    if desired != new_item.desired_stock {
                        new_item.desired_stock = desired;
                        item_changed = true;
                    }
                }
                if item_changed {
                    item_list.push(new_item);
                }
            }
        }
        if item_list.len() > 0 {
            controller.change_items(item_list);
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let inventory = &controller.state.inventory;

        let mut categories: Vec<Html> = vec![];
        for (cat_name, item_ids) in self.category_map.iter() {
            let mut item_rows: Vec<Html> = vec![];
            for item_id in item_ids {
                let item = &inventory.item_id_map[item_id];
                let (stock_ref, desired_ref) = &self.inputs[item_id];
                let item_name = 
                if item.track_general {
                    item.name.clone() + " General"
                } else {
                    item.name.clone()
                };
                item_rows.push(html!(<tr key={item_name.clone()}>
                    <td class="name">{item_name.clone()}</td>
                    <td class="stock"><input type="number" size="5" min="0" placeholder={item.stock.to_string()} ref={stock_ref} /></td>
                    <td class="desired-stock"><input type="number" size="5" min="0" placeholder={item.desired_stock.to_string()} ref={desired_ref} /></td>
                </tr>));
            }

            categories.push(html!(<td class="category" key={cat_name.clone()}>
                <h3>{cat_name.clone()}</h3>
                <hr />
                <table class="item-table">
                    {for item_rows}
                </table>
            </td>));
        }

        let mut rows: Vec<Html> = vec![];
        let mut row: Vec<Html> = vec![];
        for i in 0..categories.len() {
            let cat = categories[i].clone();
            row.push(cat);
            if  (i > 1 && (i % 3) == 2) || i == categories.len()-1 {
                rows.push(html!(<tr>
                    {for row}
                </tr>));
                row = vec![];
            }
        }

        html!(<>
            <button style="width: 66%; height: 3em" onclick={ctx.link().callback(|_| ReInventoryMsg::Submit)}>{"Submit"}</button>
            <table id="item-list">
                {for rows}
            </table>
        </>)
    }
}

