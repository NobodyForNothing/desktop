use kalk::kalk_value::ScientificNotationFormat;
use kalk::parser;
use crate::frb_generated::RustOpaque;


pub fn interpret(equation: String) -> Option<String> {
    let res = parser::eval(
        &mut parser::Context::new(),
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

