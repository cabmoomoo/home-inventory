use std::collections::BTreeMap;

use yew::prelude::*;

use crate::{models::Item, InvCont, ItemCategory};

#[function_component]
pub fn ShoppingList() -> Html {
    let inv_cont = use_context::<InvCont>().expect("no ctx found");
    let items = &inv_cont.state.items;
    let mut category_map: BTreeMap<String, Vec<Item>> = BTreeMap::new();
    for item in items {
        if item.desired_stock == 0 || item.stock >= item.desired_stock {
            continue
        }
        let cat_fetch = category_map.get_mut(&item.category);
        match cat_fetch {
            Some(cat) => {cat.push(item.clone());},
            None => {category_map.insert(item.category.clone(), vec![item.clone()]);},
        }
    }
    // let categories: Html = category_map
    //     .iter()
    //     .map(|(name, items)| html!(<ItemCategory name={name.clone()} items={items.clone()} />))
    //     .collect();

    let mut categories: Vec<Html> = vec![];
    for (name, cat_items) in &category_map {
        categories.push(html!(
            <ItemCategory name={name.clone()} items={cat_items.clone()} />
        ));
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

    html!(
        <div id="item-list">
            {rows}
        </div>
    )
}