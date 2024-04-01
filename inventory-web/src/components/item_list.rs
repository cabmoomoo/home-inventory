use std::collections::BTreeMap;

use yew::prelude::*;

use crate::{models::Item, InvCont, ItemCategory};

#[derive(Properties, PartialEq)]
pub struct ItemListProps {
    pub controller: InvCont
}

#[function_component]
pub fn ItemList(props: &ItemListProps) -> Html {
    let items = &props.controller.state.items;
    let mut category_map: BTreeMap<String, Vec<Item>> = BTreeMap::new();
    for item in items {
        let cat_fetch = category_map.get_mut(&item.category);
        match cat_fetch {
            Some(cat) => {cat.push(item.clone());},
            None => {category_map.insert(item.category.clone(), vec![item.clone()]);},
        }
    }
    let categories: Vec<Html> = category_map
        .iter()
        .map(|(name, items)| html!(<ItemCategory name={name.clone()} items={items.clone()} />))
        .collect();

    let mut rows: Vec<Html> = vec![];
    let mut row: Vec<Html> = vec![];
    for i in 0..categories.len() {
        let cat = &categories[i];
        row.push(cat.clone());
        if  (i > 1 && (i % 3) == 2) || i == categories.len()-1 {
            rows.push(html!(<tr>
                {for row}
            </tr>));
            row = vec![];
        }
    }

    html!(
        <table id="item-list">
            {for rows}
        </table>
    )
}