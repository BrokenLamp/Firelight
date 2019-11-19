trait Structure {
    fn update(&mut self, plot: &mut Plot, delta: f32, state: &mut GameState) {}
    fn get_construction_speed(&self) -> f32 {
        1.
    }
    fn can_destroy(&self) -> bool {
        false
    }
    fn get_name(&self) -> String;
}
