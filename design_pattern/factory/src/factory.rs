///! FactoryMethodパターンのCreatorとConcreteCreatorを提供するモジュール
use crate::pizza::*;

/// Pizzaのトレイトオブジェクトのエイリアス
type P = Box<dyn Pizza>;

/// Creatorに相当するトレイト  
/// Tで任意のピザメニューの型を指す  
/// Tはenumを想定しているが、任意のenumであるという型特性の指定方法がわからなかった
pub trait PizzaFactory<T> {
    /// menuで指定されたピザをトレイトオブジェクトとして返す  
    /// 具体的な実装はPizzaFactoryを実装する型に移譲する
    fn create_pizza(&self, menu: T) -> P;
    /// 客に提供するピザ注文インターフェース
    fn order_pizza(&self, menu: T) {
        // 製法は分からないがメニューに対応する何らかのピザを得る
        let pizza = self.create_pizza(menu);
        println!("===========================================");
        println!("{} ordered!!", pizza.name());
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxing();
        println!("===========================================");
    }
}

pub mod newyork {
    use crate::factory::*;
    use crate::pizza::newyork::*;

    /// ニューヨークのとあるピザ店
    pub struct NewYorkPizzaStore;
    /// ニューヨークのピザ店にピザの製法を実装する
    impl PizzaFactory<NewYorkStylePizzaMenu> for NewYorkPizzaStore {
        /// メニューで指定されたものに対応するピザを返す
        fn create_pizza(&self, menu: NewYorkStylePizzaMenu) -> P {
            match menu {
                NewYorkStylePizzaMenu::NYStyleCheesePizza => Box::new(NYStleCheesePizza {}),
                NewYorkStylePizzaMenu::NYStyleMargheritaPizza => {
                    Box::new(NYStyleMargheritaPizza {})
                }
                NewYorkStylePizzaMenu::NYStylePepperoniPizza => Box::new(NYStylePepperoniPizza {}),
            }
        }
    }
}
