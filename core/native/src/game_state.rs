use super::item::ItemBag;
use super::plot_grid::PlotGrid;

pub struct GameState {
    grid: PlotGrid,
    items: ItemBag,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            grid: PlotGrid::generate(),
            items: ItemBag::new(),
        }
    }
    pub fn update(&mut self) {
        let plots = self.grid.iter_mut();
        for plot in plots {
            plot.update(&mut self.items);
        }
    }
}
