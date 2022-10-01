///! デザインパターンのデコレータパターンを実装してみる

/// コーヒーショップのメニューが提供するインターフェース定義
trait Item {
    /// レシート明細  
    /// 何をいくら分注文したのか
    fn detail(&self) -> String;
    /// 価格
    fn price(&self) -> f32;
}

/// ドリンクメニューに追加するトッピング
/// JavaやC++で説明されるデコレータパターンではこの構造体を継承して各種トッピング毎にクラスを作るが、  
/// Rustにはフィールドのある構造体など、実体型を継承する機能は無い  
/// あくまでもインターフェースを定義したTraitを構造体が実装するのみ  
/// なのでここからどのように具体的なトッピングの実体を定義し、  
/// トッピング対象を如何にしてラッピング(=デコレート)するかが課題となった  
struct Topping<T: Item> {
    /// トッピングを追加するベースとなる任意のアイテム
    item: T,
    /// トッピングの名前
    name: String,
    /// トッピングの価格
    price: f32,
}

/// トッピングに明細と価格を実装する
impl<T: Item> Item for Topping<T> {
    fn detail(&self) -> String {
        // 自分自身の明細(トッピングメニュー名と価格)
        let self_detail = format!("{}: ${}", self.name, self.price);
        // ベースとなるアイテムに自分自身の明細を合成して返す
        format!("{}\n{}", self.item.detail(), self_detail)
    }
    fn price(&self) -> f32 {
        // ベースとなるアイテムの価格に自分自身の価格を加算して返す
        self.item.price() + self.price
    }
}

/// Topping構造体を継承するような従来のデコレータパターンでの実装はRustでは再現できない  
/// そのため具体的なトッピングはTraitに定義する関数という形で表現してみた  
trait ToppingMenu<T: Item> {
    /// ミルクトッピングを追加したものを返す
    fn milk(self) -> Topping<T>;
    /// モカトッピングを追加したものを返す
    fn mocha(self) -> Topping<T>;
}

/// Itemトレイトを実装する全ての型TにToppingMenuトレイトを実装する
/// これによりToppingとDrinkMenuが同一のトッピング追加インターフェースを持つようになる
/// トッピング追加関数自身が新たなトッピングを返すことで、  
/// builderパターンのようにチェーンメソッド形式でコードを記述できるようになる  
/// 新しいトッピングが増えたらToppingMenuトレイトとここのデフォルト実装に追加するだけでよい  
impl<T: Item> ToppingMenu<T> for T {
    fn milk(self) -> Topping<T> {
        Topping {
            // 現在の自分自身をベースとして所有権ごと移す
            item: self,
            // ミルクトッピング
            name: "milk topping".to_string(),
            // 価格は$3.3
            price: 3.3,
        }
    }
    fn mocha(self) -> Topping<T> {
        Topping {
            item: self,
            name: "mocha topping".to_string(),
            price: 4.5,
        }
    }
}

/// トッピングのベースとなるドリンクメニュー
struct DrinkMenu {
    /// ドリンクメニューの名前
    name: String,
    /// 価格
    price: f32,
}

/// Itemトレイトをドリンクメニューにも実装する
impl Item for DrinkMenu {
    fn detail(&self) -> String {
        format!("{}: ${}", self.name.clone(), self.price)
    }
    fn price(&self) -> f32 {
        self.price
    }
}

pub fn drinkmenu_decorator_sample() {
    // ベースとなるドリンクメニューを決める
    let coffee = DrinkMenu {
        name: "coffee drink".to_string(),
        price: 10.0,
    };
    // ベースに選んだドリンクメニューに好きなだけトッピングを追加する
    // ToppingMenuトレイトの関数を任意の数だけ任意の順番でメソッドチェーン形式で呼び出せば良い
    let item = coffee.milk().mocha();
    // 確定した注文内容を確認する
    /**
     * `cargo run`で実行すると以下のような出力が得られる
     *
     * ```
     * wellcome, coffee shop!
     * --------------------
     * coffee drink: $10
     * milk topping: $3.3
     * mocha topping: $4.5
     * --------------------
     * total: $17.8
     * ```
     */
    println!("--------------------");
    println!("{}", item.detail());
    println!("--------------------");
    println!("total: ${}", item.price());
}
