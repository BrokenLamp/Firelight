use super::plot::Plot;

const SIZE: usize = 15;

pub struct PlotGrid {
    plots: [Plot; SIZE * SIZE],
}

impl PlotGrid {
    fn get_plot(&mut self, x: usize, y: usize) -> Result<&mut Plot, &str> {
        if x < 0 || x > SIZE || y < 0 || y > SIZE {
            return Err("out_of_bounds");
        }
        Ok(&mut self.plots[x + y * SIZE])
    }
    fn get_size() -> usize {
        SIZE
    }
}
