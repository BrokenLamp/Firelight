use std::mem;
use super::structure::Structure;
use super::item::ItemBag;

pub struct Plot {
    bots: u32,
    level: u8,
    structure: Option<Box<Structure>>,
    constructed_percent: f32,
}

impl Plot {
    fn get_bots(&self) -> u32 {
        self.bots
    }
    fn change_bots(&mut self, num: i32) -> Result<(), ()> {
        if self.bots as i32 + num >= 0 {
            self.bots = (self.bots as i32 + num) as u32;
            Ok(())
        } else {
            Err(())
        }
    }
    fn update(&mut self, items: &mut ItemBag) {
        if let Some(structure) = &mut self.structure {
            let structure = &mut *structure;
            if self.constructed_percent > 100.0 {
                structure.update(self.bots, items);
            } else {
                self.constructed_percent += structure
                    .get_construction_speed()
                    * self.bots as f32;
            }
        }
    }
    fn has_structure(&self) -> bool {
        self.structure.is_some()
    }
    fn remove_structure(&mut self) -> Result<Box<Structure>, &str> {
        match self.structure {
            Some(_) => Ok(mem::replace(&mut self.structure, None).unwrap()),
            None => Err("no_structure"),
        }
    }
    fn set_structure(&mut self, structure: Box<Structure>, constructed_percent: f32) -> Result<(), &str> {
        if let Some(_) = self.structure {
            return Err("struct_exists");
        }
        self.structure = Some(structure);
        self.constructed_percent = constructed_percent;
        Ok(())
    }
}
