mod factory;
mod pizza;
use crate::factory::newyork::NewYorkPizzaStore;
use crate::factory::PizzaFactory;
use crate::pizza::newyork::NewYorkStylePizzaMenu;

fn main() {
    println!("welcome, pizza factory!!");
    let newyork_store = NewYorkPizzaStore {};
    newyork_store.order_pizza(NewYorkStylePizzaMenu::NYStyleMargheritaPizza);
    newyork_store.order_pizza(NewYorkStylePizzaMenu::NYStyleCheesePizza);
    newyork_store.order_pizza(NewYorkStylePizzaMenu::NYStylePepperoniPizza);
}
