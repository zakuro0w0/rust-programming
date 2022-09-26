mod state;
use crate::state::State;
use crate::state::*;

fn main() {
    println!("Hello, world!");
    let mut current: Box<dyn State> = Box::new(state::Off);
    current = current.switch();
    current = current.switch();
    current = current.switch();
}
