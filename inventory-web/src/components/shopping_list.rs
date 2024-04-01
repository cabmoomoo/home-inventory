use std::collections::BTreeMap;

use yew::prelude::*;

use crate::{models::Item, InvCont, ItemCategory};

#[derive(Properties, PartialEq)]
pub struct ShoppingListProps {
    pub controller: InvCont
}

#[function_component]
pub fn ShoppingList(props: &ShoppingListProps) -> Html {
    let items = &props.controller.state.items;
    let mut category_map: BTreeMap<String, Vec<Item>> = BTreeMap::new();
    for item in items {
        if item.desired_stock == 0 || item.stock > item.desired_stock {
            continue
        }
        let cat_fetch = category_map.get_mut(&item.category);
        match cat_fetch {
            Some(cat) => {cat.push(item.clone());},
            None => {category_map.insert(item.category.clone(), vec![item.clone()]);},
        }
    }
    let categories: Html = category_map
        .iter()
        .map(|(name, items)| html!(<ItemCategory name={name.clone()} items={items.clone()} />))
        .collect();

    html!(
        <div id="item-list">
            {categories}
        </div>
    )
}