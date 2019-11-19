#[macro_use]
extern crate neon;

mod item;
mod plot;
mod plot_grid;
mod structure;
mod structures;

use std::collections::HashMap;
use neon::prelude::*;
use item::{ Item, ItemBag };
use plot_grid::PlotGrid;


struct GameState {
    grid: PlotGrid,
    items: ItemBag,
}

impl GameState {
    
}

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(178.5))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
