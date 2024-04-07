use yew::prelude::*;

use crate::models::Item;

#[derive(Properties, PartialEq)]
pub struct ItemCategoryProps {
    pub name: String,
    pub items: Vec<Item>
}

#[function_component]
pub fn ItemCategory(props: &ItemCategoryProps) -> Html{
    let name = &props.name;
    let mut items = props.items.clone();
    items.sort_by(|a, b| a.name.cmp(&b.name));
    let mut item_rows: Vec<Html>= vec![];
    for item in items {
        let row_class = if item.desired_stock == 0 {
            None
        } else if item.stock < item.desired_stock {
            Some("stock-critical")
        } else if item.stock == item.desired_stock {
            Some("stock-low")
        } else {
            None
        };
        if item.track_general {
            let general_msg = if item.stock == 2 {
                "Good"
            } else if item.stock == 1 {
                "Low"
            } else {
                "Out"
            };
            item_rows.push(html!(<tr class={classes!(row_class)} key={item.name.clone()}>
                <td class="name">{item.name}</td>
                <td colspan="2" class="track-general">{general_msg}</td>
            </tr>));
        } else {
            item_rows.push(html!(<tr class={classes!(row_class)} key={item.name.clone()}>
                <td class="name">{item.name}</td>
                <td class="stock">{item.stock}</td>
                <td class="desired-stock">{format!(": {}", item.desired_stock)}</td>
            </tr>));
        }
    }
    html!(<td class="category" key={name.clone()}>
        <h3>{name}</h3>
        <hr />
        <table class="item-table">
            {for item_rows}
        </table>
    </td>)
}