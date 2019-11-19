#[macro_use]
extern crate neon;

use neon::prelude::*;

struct GameState {
    items: HashMap<Item, u32>,
};

impl GameState {
    
}

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(178.5))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
