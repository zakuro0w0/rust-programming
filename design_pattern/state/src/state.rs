pub trait State {
    fn switch(&self) -> Box<dyn State>;
}
pub struct On;
pub struct Off;

impl State for On {
    fn switch(&self) -> Box<dyn State> {
        println!("On::switch!!");
        Box::new(Off)
    }
}

impl State for Off {
    fn switch(&self) -> Box<dyn State> {
        println!("Off::switch!!");
        Box::new(On)
    }
}
