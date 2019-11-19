struct Mine {
    item: Item,
    items_left: u32,
    capacity: u32,
    speed: u8,
}

impl Mine {
    fn new(item: Item, num_items: u32) -> Result<Self, ()> {
        Mine {
            item: item,
            items_left: num_items,
            capacity: num_items,
        }
    }
}

impl Structure for Mine {
    fn update(&mut self, plot: &mut Plot, state: &mut GameState) {
        let change = delta.ceil() as u32 * plot.bots * speed;
        if let Some(num_items) game_state.items.get_mut(self.item) {
            *num_items += change;
        } else {
            game_state.items.insert(self.item, change);
        }
    }

    fn get_construction_speed(&mut self) -> f32 {
        0.
    }

    fn can_destroy(&self) -> bool {
        items_left == 0
    }

    fn get_name(&self) -> String {
        format!("Mine:{}", self.item)
    }
}
