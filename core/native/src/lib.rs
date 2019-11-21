#[allow(unused_imports)]
#[macro_use]
extern crate neon;

pub mod game_state;
pub mod item;
pub mod plot;
pub mod plot_grid;
pub mod structure;
pub mod structures;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(178.5))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
