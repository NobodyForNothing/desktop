use std::cell::RefCell;
use kalk::kalk_value::ScientificNotationFormat;
use kalk::parser;
use crate::frb_generated::RustOpaque;

thread_local!(static CONTEXT: parser::Context = RefCell::new(parser::Context::new()));

pub fn interpret(equation: String) -> Option<String> {
    let mut context: parser::Context = parser::Context::new();
    CONTEXT.with(|c|  c.clone_into(&mut &context));
    let res = parser::eval(
        &mut context,
        equation.as_str(),
        32
    );
    let res = res.unwrap_or(None)?;
    Some(res.to_string_pretty_format(ScientificNotationFormat::Normal))
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

