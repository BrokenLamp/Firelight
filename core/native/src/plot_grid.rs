use super::plot::Plot;

const SIZE: usize = 15;

pub struct PlotGrid {
    plots: [Plot; SIZE * SIZE],
}

impl PlotGrid {
    pub fn generate() -> Self {

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
        &mut self.plots[self.plots.len() / 2]
    }
    pub fn iter(&mut self) -> Iter<Plot> {
        self.plots.into_iter()
    }
}
