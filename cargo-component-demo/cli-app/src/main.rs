cargo_component_bindings::generate!();

use crate::bindings::component::adder::add::add;
use crate::bindings::component::calculator::calculate::calculate;
use crate::bindings::component::calculator::increment::increment;

fn main() {
    let sum = add(1, 3);
    println!("1 + 3 = {}", sum);

    let sum2 = calculate(2, 3);
    println!("2 + 3 = {}", sum2);

    let sum3 = increment(9);
    println!("9++ = {}", sum3);

    println!("Hello, world!");
}
