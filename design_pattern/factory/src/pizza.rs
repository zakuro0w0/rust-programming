/// ピザを作る工程
trait Pizza {
    /// 下ごしらえ
    fn prepare(&self) {}
    /// 生地を焼く
    fn bake(&self) {}
    /// 切り分ける
    fn cut(&self) {}
    /// 箱に詰める
    fn boxing(&self) {}
}

/// ピザメニュー
pub enum PizzaMenu {
    /// チーズピザ
    CheesePizza,
    /// ペパロニピザ
    PepperoniPizza,
    /// マルゲリータピザ
    MargheritaPizza,
    /// 野菜ピザ
    VegiePizza,
    /// ギリシャピザ
    GreekPizza,
}

// ピザメニュー毎に構造体とPizzaトレイトの実装を宣言する
struct CheesePizza {}
struct PepperoniPizza {}
struct MargheritaPizza {}
struct VegiePizza {}
struct GreekPizza {}

impl Pizza for CheesePizza {}
impl Pizza for PepperoniPizza {}
impl Pizza for MargheritaPizza {}
impl Pizza for VegiePizza {}
impl Pizza for GreekPizza {}

/// Pizzaのトレイトオブジェクトのエイリアス
type P = Box<dyn Pizza>;

/// ピザファクトリ  
/// ピザのインスタンス化コードをこの構造体に閉じ込める
struct PizzaFactory;

impl PizzaFactory {
    /// menuで指定された種類のピザを新規に作って返す
    /// 返すピザはBox<dyn Pizza>、つまりトレイトオブジェクトのBoxとなる
    fn create_pizza(menu: PizzaMenu) -> P {
        match menu {
            PizzaMenu::CheesePizza => Box::new(CheesePizza {}),
            PizzaMenu::PepperoniPizza => Box::new(PepperoniPizza {}),
            PizzaMenu::MargheritaPizza => Box::new(MargheritaPizza {}),
            PizzaMenu::VegiePizza => Box::new(VegiePizza {}),
            PizzaMenu::GreekPizza => Box::new(GreekPizza {}),
        }
    }
}

/// ピザメニューを1つ指定してピザを注文する
pub fn order_pizza(menu: PizzaMenu) {
    // どのピザを作るかを決める
    // 製法はPizzaFactoryに移譲したのでピザの種類を指定するだけで良い
    let pizza: P = PizzaFactory::create_pizza(menu);
    pizza.prepare();
    pizza.bake();
    pizza.cut();
    pizza.boxing();
}
