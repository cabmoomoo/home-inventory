

use yew::prelude::*;

use crate::models::Item;

#[derive(Properties, PartialEq)]
pub struct ItemCategoryProps {
    pub name: String,
    pub items: Vec<Item>
}

#[function_component]
pub fn ItemCategory(ItemCategoryProps { name, items }: &ItemCategoryProps) -> Html{
    let mut items = items.clone();
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
        item_rows.push(html!(<tr class={classes!(row_class)}>
            <td class="name">{item.name}</td>
            <td class="stock">{item.stock}</td>
        </tr>));
    }
    html!(<td class="category">
        <h3>{name}</h3>
        <hr />
        <table class="item-table">
            {for item_rows}
        </table>
    </td>)
}