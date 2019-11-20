use super::item::{ Item, ItemBag };
use super::plot::Plot;
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
        self.grid.iter().for_each(|&mut plot| {
            plot.update(&mut self.items);
        });
    }
}