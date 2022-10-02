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

/// ピザメニューを1つ指定してピザを注文する
pub fn order_pizza(menu: PizzaMenu) {
    // どのピザを作るかを決める
    // このコードがピザメニューの変更に対して閉じていない
    // ピザメニューが増える度にここに分岐とピザの作り方を追加しなければならない
    // 実際には構造体のインスタンス化はもっと複雑で長いコードになることが予想される
    let pizza: P = match menu {
        PizzaMenu::CheesePizza => Box::new(CheesePizza {}),
        PizzaMenu::PepperoniPizza => Box::new(PepperoniPizza {}),
        PizzaMenu::MargheritaPizza => Box::new(MargheritaPizza {}),
        PizzaMenu::VegiePizza => Box::new(VegiePizza {}),
        PizzaMenu::GreekPizza => Box::new(GreekPizza {}),
    };
    pizza.prepare();
    pizza.bake();
    pizza.cut();
    pizza.boxing();
}
