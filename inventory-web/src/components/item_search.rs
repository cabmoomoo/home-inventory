use std::collections::BTreeMap;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::InvCont;

pub enum ItemSearchMsg {
    SearchChange,
    SelectItem(AttrValue),
    SelectFirst
}

#[derive(Properties, PartialEq)]
pub struct ItemSearchProps {
    pub selection_callback: Callback<AttrValue>
}

pub struct ItemSearch {
    search_node: NodeRef,
    search_value: Option<String>,
    filtered_items: Vec<(AttrValue, AttrValue)>
}

impl Component for ItemSearch {
    type Message = ItemSearchMsg;

    type Properties = ItemSearchProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { search_node: NodeRef::default(), search_value: None, filtered_items: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (controller, _) = ctx.link().context::<InvCont>(Callback::noop()).expect("no ctx found");
        let props = ctx.props();
        let inventory = &controller.state.inventory;

        match msg {
            ItemSearchMsg::SearchChange => {
                let search_value = self.search_node.cast::<HtmlInputElement>().unwrap().value();
                if !search_value.is_empty() {
                    self.search_value = Some(search_value);
                    self.filtered_items = filter_items(&inventory.name_to_id, self.search_value.clone().unwrap());
                }
            },
            ItemSearchMsg::SelectItem(item_id) => {
                props.selection_callback.emit(item_id);
                let node = self.search_node.cast::<HtmlInputElement>().unwrap();
                node.set_value("");
                self.search_value = None;
            },
            ItemSearchMsg::SelectFirst => {
                let (_item_name, item_id) = &self.filtered_items[0];
                props.selection_callback.emit(item_id.clone());
                let node = self.search_node.cast::<HtmlInputElement>().unwrap();
                node.set_value("");
                self.search_value = None;
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut search_items: Vec<Html> = vec![];
        if self.search_value.is_some() {            
            for (item_name, item_id) in self.filtered_items.iter() {
                let id = item_id.clone();
                // let callback = Callback::from(move |_| ItemSearchMsg::SelectItem(id));
                search_items.push(html!(<div class="search-item" onclick={ctx.link().callback(move |_| ItemSearchMsg::SelectItem(id.clone()))}>
                    <p>{item_name.clone()}</p>
                </div>));
            }
        }
        
        html!(<div class="item-search">
            <input class="search" placeholder="Search" ref={&self.search_node} 
                // onkeyup={ctx.link().callback(|_| ItemSearchMsg::SearchChange)}
                onkeyup={ctx.link().callback(|e: KeyboardEvent| {
                    if e.key().eq("Enter") {
                        ItemSearchMsg::SelectFirst
                    } else {
                        ItemSearchMsg::SearchChange
                    }
                })}
            />
            <div class="search-items">
                {for search_items}
            </div>
        </div>)
    }
}

/// Takes the name_to_id map from inventory and returns a vector of (name, id) filtered by the search_value.
/// search_value is made lowercase and broken up by split_ascii_whitespace() to make for a fuzzier search.
/// 
/// # Examples
/// 
/// Pseudo-code example:
/// ```
/// item_name = "Dr Pepper Large 12oz Bottle";
/// search_value = "pep can";
/// assert!(item_name.contains(search_value));
/// ```
pub fn filter_items<'a>(items: &'a BTreeMap<AttrValue, AttrValue>, search_value: String) -> Vec<(AttrValue, AttrValue)> {
    let mut filters = vec![];
    for part in search_value.split_ascii_whitespace() {
        let part = part.to_lowercase();
        filters.push(move |a: &&AttrValue| a.to_lowercase().contains(&part));
    }
    let mut filtered_items = vec![];
    let mut i = 0;
    'outer: for (item_name, item_id) in items.iter() {
        if i >= 5 {
            break;
        }
        for filter in filters.iter() {
            if !filter(&item_name) {
                continue 'outer;
            }
        }
        filtered_items.push((item_name.clone(), item_id.clone()));
        i += 1;
    }
    filtered_items
}