mod singleton;
use crate::singleton::*;

fn main() {
    println!("Hello, world!");
    singleton::new().print();
    singleton::new().print();
    singleton::new().print();
}
