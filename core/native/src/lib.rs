#[macro_use]
extern crate neon;

mod item;
mod plot;
mod plot_grid;
mod structure;

use std::collections::HashMap;
use neon::prelude::*;
use item::Item;
use plot_grid::PlotGrid;


struct GameState {
    grid: PlotGrid,
    items: HashMap<Item, u32>,
}

impl GameState {
    
}

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(178.5))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
