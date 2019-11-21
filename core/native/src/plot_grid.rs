use std::slice::IterMut;
use super::plot::Plot;

const SIZE: usize = 15;

pub struct PlotGrid {
    plots: Vec<Plot>,
}

impl PlotGrid {
    pub fn generate() -> Self {
        PlotGrid {
            plots: Vec::with_capacity(SIZE * SIZE),
        }
    }
    pub fn get_plot(&mut self, x: usize, y: usize) -> Result<&mut Plot, &str> {
        if x > SIZE || y > SIZE {
            return Err("out_of_bounds");
        }
        Ok(&mut self.plots[x + y * SIZE])
    }
    pub fn get_size() -> usize {
        SIZE
    }
    pub fn get_home(&mut self) -> &mut Plot {
        &mut self.plots[SIZE * SIZE / 2]
    }
    pub fn iter_mut(&mut self) -> IterMut<Plot> {
        self.plots.iter_mut()
    }
}
