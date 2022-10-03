/// ピザを作る工程
pub trait Pizza {
    /// ピザのメニュー名
    fn name(&self) -> String;
    /// 一連のトッピング
    fn toppings(&self) -> Vec<String>;
    /// 下ごしらえ
    fn prepare(&self) {
        println!("{}を下準備", self.name());
        println!("生地をこねる");
        println!("ソースを追加");
        println!("トッピングを追加");
        for topping in self.toppings() {
            println!("  - {}", topping);
        }
    }
    /// 生地を焼く
    fn bake(&self) {
        println!("180℃で25分間焼く");
    }
    /// 切り分ける
    fn cut(&self) {
        println!("ピザを扇形にカットする");
    }
    /// 箱に詰める
    fn boxing(&self) {
        println!("PizzaStoreの箱にピザを入れる");
    }
}

pub mod newyork {
    use crate::pizza::Pizza;

    /// ニューヨークのとあるピザ店のメニュー
    pub enum NewYorkStylePizzaMenu {
        NYStyleCheesePizza,
        NYStyleMargheritaPizza,
        NYStylePepperoniPizza,
    }

    pub struct NYStleCheesePizza;
    pub struct NYStyleMargheritaPizza;
    pub struct NYStylePepperoniPizza;

    impl Pizza for NYStleCheesePizza {
        fn name(&self) -> String {
            "NYStyleCheesePizza".to_string()
        }
        fn toppings(&self) -> Vec<String> {
            vec!["NY cheese".to_string(), "NY cheese2".to_string()]
        }
    }

    impl Pizza for NYStyleMargheritaPizza {
        fn name(&self) -> String {
            "NYStyleMargheritaPizza".to_string()
        }
        fn toppings(&self) -> Vec<String> {
            vec!["NY tomato".to_string(), "NY basil".to_string()]
        }
    }

    impl Pizza for NYStylePepperoniPizza {
        fn name(&self) -> String {
            "NYStylePepperoniPizza".to_string()
        }
        fn toppings(&self) -> Vec<String> {
            vec!["NY pepperoni".to_string(), "NY pepperoni2".to_string()]
        }
    }
}
