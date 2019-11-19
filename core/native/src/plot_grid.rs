
const SIZE: isize = 15;

struct PlotGrid {
    plots: [mut Plot; SIZE * SIZE],
}

impl PlotGrid {
    fn get_plot(&mut self, x: isize, y: isize) -> Result<&mut Plot, &str> {
        if x < 0 || x > SIZE || y < 0 || y > SIZE {
            return Err(format!("off_grid:{},{}", x, y));
        }
        self.plots[x + y * SIZE]
    }
}
