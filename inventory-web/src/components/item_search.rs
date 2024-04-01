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
    pub controller: InvCont,
    pub selection_callback: Callback<AttrValue>
}

pub struct ItemSearch {
    search_node: NodeRef,
    search_value: Option<String>
}

impl Component for ItemSearch {
    type Message = ItemSearchMsg;

    type Properties = ItemSearchProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { search_node: NodeRef::default(), search_value: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        let inventory = &props.controller.state.inventory;

        match msg {
            ItemSearchMsg::SearchChange => {
                let node = self.search_node.cast::<HtmlInputElement>().unwrap();
                self.search_value = Some(node.value());
            },
            ItemSearchMsg::SelectItem(item_id) => {
                props.selection_callback.emit(item_id);
                let node = self.search_node.cast::<HtmlInputElement>().unwrap();
                node.set_value("");
                self.search_value = None;
            },
            ItemSearchMsg::SelectFirst => {
                let search_value = self.search_value.clone().unwrap().to_lowercase();
                let option = inventory.name_to_id.iter().filter(|(a, _)| a.to_lowercase().contains(&search_value)).next();
                let (_item_name, item_id) = option.unwrap();
                props.selection_callback.emit(item_id.clone());
                let node = self.search_node.cast::<HtmlInputElement>().unwrap();
                node.set_value("");
                self.search_value = None;
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let inventory = &props.controller.state.inventory;

        let mut search_items: Vec<Html> = vec![];
        if self.search_value.is_some() {
            // let search_value = AttrValue::from(self.search_value.clone().unwrap());
            let search_value = self.search_value.clone().unwrap().to_lowercase();
            let mut i = 0;
            for (item_name, item_id) in inventory.name_to_id.iter().filter(|(a, _)| a.to_lowercase().contains(&search_value)) {
                if i >= 5 {
                    break;
                }
                let id = item_id.clone();
                // let callback = Callback::from(move |_| ItemSearchMsg::SelectItem(id));
                search_items.push(html!(<div class="search-item" onclick={ctx.link().callback(move |_| ItemSearchMsg::SelectItem(id.clone()))}>
                    <p>{item_name.clone()}</p>
                </div>));
                i += 1;
            }
        }
        
        html!(<div class="item-search">
            <input class="search" placeholder="Search" ref={&self.search_node} 
                // onkeyup={ctx.link().callback(|_| ItemSearchMsg::SearchChange)}
                onkeypress={ctx.link().callback(|e: KeyboardEvent| {
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