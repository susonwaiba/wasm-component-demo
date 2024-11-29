#[allow(warnings)]
mod bindings;

use bindings::exports::component::calculator::calculate::Guest;

use bindings::component::adder::add::add;

struct Component;

impl Guest for Component {
    fn eval_expression(_expr: String) -> u32 {
        // Cleverly parse `expr` into values and operations, and evaluate
        // them meticulously.
        add(123, 456)
    }
}

bindings::export!(Component with_types_in bindings);
