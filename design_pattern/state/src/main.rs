mod state;
use crate::state::State;
use crate::state::*;

fn main() {
    println!("Hello, world!");
    let mut current = Box::new(&Off as &dyn State);
    current = current.switch();
    current = current.switch();
    current = current.switch();
    current = current.switch();
}
