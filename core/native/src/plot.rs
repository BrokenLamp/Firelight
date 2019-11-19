struct Plot {
    bots: u32,
    level: u8,
    structure: Option<Structure>,
    constructed_percent: f32,
}

impl Plot {
    fn get_bots(&self) -> u32 {
        self.bots
    }
    fn change_bots(&mut self, num: i32) -> Result<(), ()> {
        if self.bots + num >= 0 {
            self.bots += num;
            Ok(())
        } else {
            Err(())
        }
    }
    fn update(&mut self, delta: f32, state: &mut GameState) {
        if let Some(structure) = self.structure {
            if self.constructed_percent > 100.0 {
                structure.update(self, delta, state);
            } else {
                constructed_percent += delta
                    * self.structure.get_construction_speed()
                    * self.bots;
            }
        }
    }
    fn destroy_structure(&mut self) -> Result<Structure, &str> {
        let structure = match self.structure.ok_or("no_struct")?;
        self.structure = None;
        Ok(structure)
    }
    fn set_structure(&mut self, structure: Structure, constructed_percent: f32) -> Result<(), &str> {
        if let Some(_) = self.structure {
            return Err("struct_exists");
        }
        self.structure = Some(structure);
        self.constructed_percent = constructed_percent;
        Ok(())
    }
}
