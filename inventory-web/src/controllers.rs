use yew::{Callback, UseReducerHandle};

use crate::{error_message, items_api, models::{Item, RestockItem}, state::{ItemAction, ItemsState}, success_message, MessageContainer, MessageContainerAction};

#[derive(Clone, PartialEq)]
pub struct InventoryController {
    pub state: UseReducerHandle<ItemsState>,
    pub message: UseReducerHandle<MessageContainer>
}
// impl PartialEq for InventoryController {
//     fn eq(&self, other: &Self) -> bool {
//         self.state.items == other.state.items
//     }
// }

impl InventoryController {
    pub fn new(state: UseReducerHandle<ItemsState>, message: UseReducerHandle<MessageContainer>) -> InventoryController {
        InventoryController { state, message }
    }

    pub fn init_items(&self) {
        let items = self.state.clone();
        let message = self.message.clone();
        let inv_conv = std::rc::Rc::new(self.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::fetch_items().await;
            match response {
                Ok(fetched_items) => items.dispatch(ItemAction::Set(fetched_items)),
                Err(_) => {
                    // let inv_conv = std::rc::Rc::new(self.clone());
                    message.dispatch(MessageContainerAction::Change { 
                        name: "Error contacting database".into(),
                        message: "This web interface has successfully loaded, but can not make contact with the database.\n
                        There could be a number of reasons for this, but the most likely one is that the database failed to start or recently crashed.\n
                        Please ensure the database is running before pressing retry or reloading this page.".into(),
                        additional_actions: Some(vec![
                            (Callback::from(move |_| inv_conv.init_items()), "Retry database connection".into())
                        ])
                    });
                },
            }
        });
        // message.dispatch(MessageContainerAction::Change { 
        //     name: "Test Message".into(),
        //     message: "This is a test messsage. It is designed to test the messaging function. Please stand by as this test is performed.".into(),
        //     additional_actions: None
        // });
    }

    pub fn new_item(&self, name: String, category: String) {
        let items = self.state.clone();
        let message = self.message.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::new_item(&name, &category).await;
            match response {
                Ok(item) => {
                    message.dispatch(success_message(format!("Item {} added successfully", item.name)));
                    items.dispatch(ItemAction::Add(item));
                },
                Err(e) => message.dispatch(error_message(e.to_string())),
            }
        });
    }

    pub fn add_full_item(&self, name: String, category: String, stock: i64, desired_stock: i64) {
        let items = self.state.clone();
        let message = self.message.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::add_full_item(&name, &category, stock, desired_stock).await;
            match response {
                Ok(item) => {
                    message.dispatch(success_message(format!("Full item {} added successfully", item.name)));
                    items.dispatch(ItemAction::Add(item));
                },
                Err(e) => message.dispatch(error_message(e.to_string())),
            }
        });
    }

    pub fn change_item(&self, id: String, item: Item) {
        let message = self.message.clone();
        let inv_conv = std::rc::Rc::new(self.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::change_item(&id, item).await;
            match response {
                Ok(item) => {
                    message.dispatch(success_message(format!(
                        "Item {} changed successfully. Should now look like:\ncategory: {};\nstock: {};\ndesired_stock: {};", 
                        item.name, item.category, item.stock, item.desired_stock)));
                    inv_conv.init_items();
                },
                Err(e) => {
                    message.dispatch(error_message(e.to_string()));
                    return;
                },
            }
        });
    }

    pub fn delete_item(&self, id: String) {
        let message = self.message.clone();
        let inv_conv = std::rc::Rc::new(self.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::delete_item(&id).await;
            match response {
                Ok(_rows) => {
                    message.dispatch(success_message("An item was successfully deleted".into()));
                    inv_conv.init_items();
                },
                Err(e) => {
                    message.dispatch(error_message(e.to_string()));
                    return;
                },
            }
        });
    }

    pub fn restock_items(&self, restock: Vec<RestockItem>) {
        let message = self.message.clone();
        let inv_conv = std::rc::Rc::new(self.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::restock_items(restock).await;
            match response {
                Ok(rows) => {
                    message.dispatch(success_message(format!("{} items were successfully restocked", rows.rows_affected)));
                    inv_conv.init_items();
                },
                Err(e) => {
                    message.dispatch(error_message(e.to_string()));
                    return;
                },
            }
        });
    }

    pub fn consume_items(&self, consume: Vec<RestockItem>) {
        let message = self.message.clone();
        let inv_conv = std::rc::Rc::new(self.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = items_api::consume_items(consume).await;
            match response {
                Ok(rows) => {
                    message.dispatch(success_message(format!("{} items were successfully consumed", rows.rows_affected)));
                    inv_conv.init_items();
                },
                Err(e) => {
                    message.dispatch(error_message(e.to_string()));
                    return;
                },
            }
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