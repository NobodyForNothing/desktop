use std::cell::RefCell;
use kalk::calculation_result::CalculationResult;
use kalk::kalk_value::ScientificNotationFormat;
use kalk::parser;

thread_local!(static CONTEXT: RefCell<parser::Context> = RefCell::new(parser::Context::new()));

pub fn interpret(equation: String) -> Option<String> {
    let mut res: Option<CalculationResult> = None;
    CONTEXT.with(|c|  {
        res = parser::eval(
            &mut c.borrow_mut(),
            equation.as_str(),
            32
        ).unwrap_or(None)
    });
    let res = res?;
    Some(res.to_string_pretty_format(ScientificNotationFormat::Normal))
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

