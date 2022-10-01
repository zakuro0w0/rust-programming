trait Item {
    fn detail(&self) -> String;
    fn price(&self) -> f32;
}

struct Topping<T: Item> {
    item: T,
    name: String,
    price: f32,
}

impl<T: Item> Item for Topping<T> {
    fn detail(&self) -> String {
        let self_detail = format!("{}: ${}", self.name, self.price);
        format!("{}\n{}", self.item.detail(), self_detail)
    }
    fn price(&self) -> f32 {
        self.item.price() + self.price
    }
}

fn milk<T: Item>(item: T) -> Topping<T> {
    Topping::<T> {
        item: item,
        name: "milk topping".to_string(),
        price: 3.0,
    }
}

fn mocha<T: Item>(item: T) -> Topping<T> {
    Topping::<T> {
        item: item,
        name: "mocha topping".to_string(),
        price: 4.2,
    }
}

struct DrinkMenu {
    name: String,
    price: f32,
}

impl Item for DrinkMenu {
    fn detail(&self) -> String {
        format!("{}: ${}", self.name.clone(), self.price)
    }
    fn price(&self) -> f32 {
        self.price
    }
}

pub fn beverage_decorator_sample() {
    let coffee = DrinkMenu {
        name: "coffee drink".to_string(),
        price: 10.0,
    };
    let item = mocha(milk(coffee));
    println!("--------------------");
    println!("{}", item.detail());
    println!("--------------------");
    println!("total: ${}", item.price());
}
