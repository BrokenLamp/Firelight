use crate::item::{ Item, ItemBag };
use crate::structure::Structure;

pub struct Mine {
    item: Item,
    items_left: u32,
    capacity: u32,
}

impl Mine {
    pub fn new(item: Item, num_items: u32) -> Self {
        Mine {
            item: item,
            items_left: num_items,
            capacity: num_items,
        }
    }
}

impl Structure for Mine {
    fn update(&mut self, bots: u32, items: &mut ItemBag) {
        let change: u32 = bots;
        self.items_left = (self.items_left - change).max(0) as u32;
        items.add(self.item, change);
    }

    fn get_construction_speed(&self) -> f32 {
        0.
    }

    fn can_destroy(&self) -> bool {
        self.items_left == 0
    }

    fn get_info(&self) -> String {
        format!("Mine:{:?}:{}/{}", self.item, self.items_left, self.capacity)
    }
}
