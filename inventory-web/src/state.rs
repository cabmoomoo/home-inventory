use std::collections::BTreeMap;

use yew::{AttrValue, Reducible};

use crate::models::Item;

pub enum ItemAction {
    Set(Vec<Item>),
    Add(Item)
}

#[derive(PartialEq, Clone)]
pub struct ItemsState {
    pub items: Vec<Item>,
    pub inventory: Inventory
}

impl Default for ItemsState {
    fn default() -> Self {
        Self { items: vec![], inventory: Inventory::default() }
    }
}

impl Reducible for ItemsState {
    type Action = ItemAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut next_items = self.items.clone();
        let mut inventory = self.inventory.clone();

        match action {
            ItemAction::Set(items) => {next_items = items; inventory.make(next_items.clone())},
            ItemAction::Add(item) => next_items.push(item),
        }

        Self { items: next_items, inventory }.into()
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Inventory {
    pub name_to_id: BTreeMap<AttrValue, AttrValue>,
    pub item_id_map: BTreeMap<AttrValue, Item>
}
impl Inventory {
    pub fn make<'a>(&mut self, items: Vec<Item>) {
        let mut name_to_id = BTreeMap::new();
        let mut item_id_map = BTreeMap::new();
        for item in items {
            let name = AttrValue::from(item.name.clone());
            let id = AttrValue::from(item.id.clone());
            name_to_id.insert(name, id.clone());
            item_id_map.insert(id, item);
        }
        self.name_to_id = name_to_id;
        self.item_id_map = item_id_map;
    }
}