use yew::UseReducerHandle;

use crate::{items_api, models::{Item, RestockItem}, state::{ItemAction, ItemsState}};

#[derive(Clone)]
pub struct InventoryController {
    pub state: UseReducerHandle<ItemsState>
} impl PartialEq for InventoryController {
    fn eq(&self, other: &Self) -> bool {
        self.state.items == other.state.items
    }
}

impl InventoryController {
    pub fn new(state: UseReducerHandle<ItemsState>) -> InventoryController {
        InventoryController { state }
    }

    pub fn init_items(&self) {
        let items = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_items = items_api::fetch_items().await.unwrap();
            items.dispatch(ItemAction::Set(fetched_items));
        });
    }

    pub fn new_item(&self, name: String, category: String) {
        let items = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::new_item(&name, &category).await.unwrap();
            items.dispatch(ItemAction::Add(response));
        });
    }

    pub fn add_full_item(&self, name: String, category: String, stock: i64, desired_stock: i64) {
        let items = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::add_full_item(&name, &category, stock, desired_stock).await.unwrap();
            items.dispatch(ItemAction::Add(response));
        });
    }

    pub fn change_item(&self, id: String, item: Item) {
        wasm_bindgen_futures::spawn_local(async move {
            let _response = items_api::change_item(&id, item).await.unwrap();
        });
        self.init_items();
    }

    pub fn delete_item(&self, id: String) {
        wasm_bindgen_futures::spawn_local(async move {
            let _response = items_api::delete_item(&id).await.unwrap();
        });
        self.init_items();
    }

    pub fn restock_items(&self, restock: Vec<RestockItem>) {
        wasm_bindgen_futures::spawn_local(async move {
            let _response = items_api::restock_items(restock).await.unwrap();
        });
        self.init_items();
    }

    pub fn consume_items(&self, consume: Vec<RestockItem>) {
        wasm_bindgen_futures::spawn_local(async move {
            let _response = items_api::consume_items(consume).await.unwrap();
            // items.dispatch(ItemAction)
        });
        self.init_items();
    }

    // pub fn test_request(&self) {
    //     let items = self.state.clone();
    //     wasm_bindgen_futures::spawn_local(async move {
    //         let response = items_api::test_request().await.unwrap();
    //         items.dispatch(ItemAction::Test(response))
    //     });
    // }
}