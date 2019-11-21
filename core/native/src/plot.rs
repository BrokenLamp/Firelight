use super::item::ItemBag;
use super::structure::Structure;
use std::mem;

pub struct Plot {
    bots: u32,
    structure: Option<Box<dyn Structure>>,
    constructed_percent: f32,
    is_discovered: bool,
}

impl Plot {
    pub fn new() -> Self {
        Plot {
            bots: 0,
            structure: None,
            constructed_percent: 0.,
            is_discovered: false,
        }
    }
    pub fn get_bots(&self) -> u32 {
        self.bots
    }

    pub fn change_bots(&mut self, num: i32) -> Result<(), ()> {
        if self.bots as i32 + num >= 0 {
            self.bots = (self.bots as i32 + num) as u32;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn update(&mut self, items: &mut ItemBag) {
        if let Some(structure) = &mut self.structure {
            let structure = &mut *structure;
            if self.constructed_percent > 100.0 {
                structure.update(self.bots, items);
            } else {
                self.constructed_percent += structure.get_construction_speed() * self.bots as f32;
            }
        }
    }

    pub fn has_structure(&self) -> bool {
        self.structure.is_some()
    }

    pub fn remove_structure(&mut self) -> Result<Box<dyn Structure>, &str> {
        match self.structure {
            Some(_) => Ok(mem::replace(&mut self.structure, None).unwrap()),
            None => Err("no_structure"),
        }
    }

    pub fn get_structure(&mut self) -> Option<&mut dyn Structure> {
        self.structure
            .as_mut()
            .map(|x| &mut **x as &mut dyn Structure)
    }

    pub fn set_structure(
        &mut self,
        structure: Box<dyn Structure>,
        constructed_percent: f32,
    ) -> Result<(), &str> {
        if let Some(_) = self.structure {
            return Err("struct_exists");
        }
        self.structure = Some(structure);
        self.constructed_percent = constructed_percent;
        Ok(())
    }

    pub fn is_discovered(&self) -> bool {
        self.is_discovered
    }

    pub fn discover(&mut self) -> Result<(), &str> {
        if self.is_discovered {
            Err("already_discovered")
        } else {
            self.is_discovered = true;
            Ok(())
        }
    }
}
