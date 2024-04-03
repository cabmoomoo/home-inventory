use std::rc::Rc;
use yew::prelude::*;

mod components;
mod models;
mod state;
mod items_api;
mod controllers;

use components::*;
use controllers::*;
use state::*;

pub type InvCont = Rc<InventoryController>;

#[derive(Default, PartialEq, Clone)]
pub enum Tabs {
    #[default]
    Home,
    DinnerList,
    GroceryBag,
    ShoppingList,
    Dev
}

#[derive(Default, PartialEq, Clone)]
pub struct Tab {
    tab: Tabs
}

#[function_component(App)]
fn app() -> Html {
    let items = use_reducer_eq(ItemsState::default);
    let message_container = use_reducer_eq(MessageContainer::default);
    let inv_controller = Rc::new(InventoryController::new(items.clone(), message_container.clone()));

    {
        let inv_controller = inv_controller.clone();
        use_effect_with((), 
            move |_| {
                inv_controller.init_items();
                || ()
            }
        )
    }

    let tab = use_state_eq(Tabs::default);

    // let on_create_task = {
    //     let inv_controller = inv_controller.clone();
    //     Callback::from(move |(name, category)| {
    //         inv_controller.new_item(name, category);
    //     })
    // };

    let (mut home_tab, mut dinner_tab, mut grocery_tab, mut shopping_tab, mut dev_tab) = (None,None,None,None,None);
    match *tab {
    Tabs::Home => home_tab = Some("active"),
    Tabs::DinnerList => dinner_tab = Some("active"),
    Tabs::GroceryBag => grocery_tab = Some("active"),
    Tabs::ShoppingList => shopping_tab = Some("active"),
    Tabs::Dev => dev_tab = Some("active"),
    }

    html!(<>
        <div class="tabs">
            <button class={classes!("tab_button", home_tab)} onclick={{let tab = tab.clone(); move |_| tab.set(Tabs::Home)}}>{"Home"}</button>
            <button class={classes!("tab_button", dinner_tab)} onclick={{let tab = tab.clone(); move |_| tab.set(Tabs::DinnerList)}}>{"Dinner List"}</button>
            <button class={classes!("tab_button", grocery_tab)} onclick={{let tab = tab.clone(); move |_| tab.set(Tabs::GroceryBag)}}>{"Grocery Bag"}</button>
            <button class={classes!("tab_button", shopping_tab)} onclick={{let tab = tab.clone(); move |_| tab.set(Tabs::ShoppingList)}}>{"Shopping List"}</button>
        </div>
        <ContextProvider<InvCont> context={inv_controller.clone()}>
        <div class={classes!("tab", home_tab)}>
            <ItemList />//controller={inv_controller.clone()} items={items.items.clone()}/>
        </div>
        <div class={classes!("tab", dinner_tab)}>
            <Dinnerlist />
        </div>
        <div class={classes!("tab", grocery_tab)}>
            <GroceryBag />
        </div>
        <div class={classes!("tab", shopping_tab)}>
            <ShoppingList />
        </div>
        <div class={classes!("tab", dev_tab)}>
            <DevTab />
        </div>
        </ContextProvider<InvCont>>
        <div class="dev">
            <button class={classes!("tab_button", dev_tab)} onclick={{let tab=tab.clone(); move |_| tab.set(Tabs::Dev)}}>{"Dev"}</button>
        </div>
        <ContextProvider<MessageContainer> context={(*message_container).clone()}>
            <MessageBox />
        </ContextProvider<MessageContainer>>
    </>)
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
