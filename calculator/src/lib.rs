cargo_component_bindings::generate!();

use crate::bindings::component::adder::add::add;
use crate::bindings::exports::component::calculator::calculate::Guest;
use crate::bindings::exports::component::calculator::increment::Guest as IncrementGuest;

struct Component;

impl Guest for Component {
    fn calculate(x: u32, y: u32) -> u32 {
        add(x, y)
    }
}

impl IncrementGuest for Component {
    fn increment(x: u32) -> u32 {
        x + 1
    }
}
