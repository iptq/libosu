extern crate libosu;
#[macro_use]
extern crate neon;

mod beatmap;
mod point;

use neon::js::JsString;
use neon::vm::{Call, JsResult};

pub use beatmap::*;
pub use point::*;

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

register_module!(m, {
    m.export("hello", hello)?;
    Ok(())
});
