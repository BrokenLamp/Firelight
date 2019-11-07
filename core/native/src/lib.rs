#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(178.5))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
