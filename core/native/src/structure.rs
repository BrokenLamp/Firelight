use super::plot::Plot;
use super::GameState;

pub trait Structure {
    fn update(&mut self, bots: &mut u32, delta: f32, state: &mut GameState) {}
    fn get_construction_speed(&self) -> f32 {
        1.
    }
    fn can_destroy(&self) -> bool {
        false
    }
    fn get_name(&self) -> String;
}
