use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Item {
    Coal,
    Copper,
    CopperOre,
    InactiveBot,
    Iron,
    IronOre,
    Steel,
    Wood,
}

pub struct ItemBag {
    items: HashMap<Item, u32>,
}

impl ItemBag {
    pub fn new() -> Self {
        ItemBag {
            items: HashMap::with_capacity(64),
        }
    }

    /// Returns number crafted
    pub fn craft(&mut self, _recipe: &[(Item, u32)], _item: Item, quantity: u32) -> u32 {
        // TODO: (required) DO THIS
        quantity
    }

    pub fn add(&mut self, item: Item, quantity: u32) {
        self.set(item, self.get(item) + quantity);
    }

    /// Returns quantity that was actually removed
    pub fn remove(&mut self, _item: Item, quantity: u32) -> u32 {
        // TODO: (required) DO THIS
        quantity
    }

    pub fn get(&self, item: Item) -> u32 {
        match self.items.get(&item) {
            Some(quantity) => *quantity,
            None => 0,
        }
    }

    pub fn set(&mut self, _item: Item, _quantity: u32) {
        // TODO: (required) DO THIS
    }
}
