mod bindings;

use bindings::component::calculator::calculate::eval_expression;

fn main() {
    let result = eval_expression("1 + 1");
    println!("1 + 1 = {result}");
}
