use super::item::ItemBag;

pub trait Structure {
    fn update(&mut self, bots: u32, items: &mut ItemBag);
    fn get_construction_speed(&self) -> f32 {
        1.
    }
    fn can_destroy(&self) -> bool {
        false
    }
    fn get_info(&self) -> String;
}
