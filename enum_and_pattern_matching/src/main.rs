fn enumelated_type() {
    // C++スタイルの列挙型
    enum MyOrdering {
        // Ordering列挙型は以下の3つの値を取りうる
        // この3つの値はvariantまたは構成子(constructors)と呼ばれる
        Less,
        Equal,
        Greater,
    }
    // ↑のようなOrdering列挙型は標準ライブラリに含まれている
    // std::cmpモジュールから列挙型だけをimportする場合は以下のように書く
    use std::cmp::Ordering;
    // variantを全てimportするには以下のように書く
    // 列挙型名::の記述無しで直接variantを記述できるが明示的でなくなるので非推奨
    // use std::cmp::Ordering::{self, *};

    fn compare(n: i32, m: i32) -> Ordering {
        if n < m {
            // variantにアクセスする場合は列挙型名::variant名
            Ordering::Less
        } else if n > m {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    // C++スタイルの列挙型はメモリ上では整数値として表現される
    // 使用する整数値を指定した方が便利な場合がある
    // 指定しなければコンパイラが0から順番に値を決めてくれる
    enum HttpStatus {
        Ok = 200,
        NotFound = 404,
        ServerInternalError = 500,
    }
    // C++スタイルの列挙型から整数型へのキャストはOK
    // しかし、整数型をRust列挙型にキャストすることはできない
    // Rustでは列挙型の値は必ずvariantのいずれかになることを保証する
    assert_eq!(HttpStatus::NotFound as i32, 404);
}

// 構造体同様に==演算子等をコンパイラが提供してくれるが、
// それには明示的にderiveで要求する必要がある
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}
// 列挙型は構造体と同様にメソッドを持つこともできる
impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
}
// 意図的に丸めたタイムスタンプの列挙型
// February 9, 2016, at 9:49 AMではなく「6ヶ月前」のように表示したい
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    // ↓のような引数を持つ構成子をタプル型ヴァリアント(tuple variant)と呼ぶ
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn data_keep_enumelated_type() {
    // 「30年前」を表現するtuple variantの構築
    let a = RoughTime::InThePast(TimeUnit::Years, 30);
    // 「10時間後」を表現するtuple variantの構築
    let b = RoughTime::InTheFuture(TimeUnit::Hours, 10);

    struct Point3d {
        x: f32,
        y: f32,
        z: f32,
    }
    impl Point3d {
        const ORIGIN: Point3d = Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    // 列挙型には構造体ヴァリアント(sturct variant)を定義することもできる
    enum Shape {
        // これらのvariantは構造体と同様に名前付きフィールドを持つ
        Sphere { center: Point3d, radius: f32 },
        Cuboid { corner1: Point3d, corner2: Point3d },
    }
    // 値の構築も名前付きフィールド型構造体と変わらない
    let unit_sphere = Shape::Sphere {
        center: Point3d::ORIGIN,
        radius: 1.0,
    };
}

fn rich_data_structure() {
    use std::collections::HashMap;
    // 任意のjsonドキュメントを表現するための型の例
    // json標準は様々なデータ型を規定している
    // null, 真偽値, 数値, 文字列, json値の配列, 文字列キーに対応するjson値のマップ
    // ほとんど同じものがRust構造体をシリアライズするserde_jsonライブラリに存在する
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String, Json>>),
    }
    let a = Json::String("value".to_string());
    let b = Json::Number(99.0);
    let c = Json::Array(vec![a, b]);
}

fn generic_enumelated_type() {
    // 列挙型をジェネリックにすることができる
    // これまでに見たOption型やResult型もジェネリック列挙型の1つ
    enum MyOption<T> {
        None,
        Some(T),
    }
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // ジェネリックデータ構造を数行のコードで書いた例
    // ↓のBinaryTreeとTreeNodeの定義だけで、任意個数の型Tのデータを保持できる
    enum BinaryTree<T> {
        // データを持っていない場合
        Empty,
        // データを持っている場合
        // ヒープ上に置かれたTreeNodeへのポインタであるBoxをデータとして持つ
        NoneEmpty(Box<TreeNode<T>>),
    }
    // BinaryTreeを構成する要素
    struct TreeNode<T> {
        // 実際の要素
        element: T,
        // left, rightはサブツリーを示す
        // 左右の2方向に分岐する二分木ツリーとなる
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    let jupiter_tree = BinaryTree::NoneEmpty(Box::new(TreeNode {
        element: "jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));
    let mars_tree = BinaryTree::NoneEmpty(Box::new(TreeNode {
        element: "mars",
        left: jupiter_tree,
        right: BinaryTree::Empty,
    }));
}

fn pattern_matching() {
    fn rough_time_to_english(rt: RoughTime) -> String {
        match rt {
            // 例えばrtがRoughTime::InTheFuture(TimeUnit::Months, 1)だった場合
            // InThePastのパターンにはマッチしない
            RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
            // JustNowのパターンにもマッチしない
            RoughTime::JustNow => format!("just now"),
            // InTheFutureのパターンにマッチする
            // パターンにunitsやcountのような変数名が入っている場合、パターン以降のコードでローカル変数となる
            // ここではrtのTimeUnit::Monthsがunitsに、1がcountに入る
            RoughTime::InTheFuture(units, count) => {
                format!("{} {} from now", count, units.plural())
            }
        }
    }
}

fn literal_variable_wildcard() {
    let a = 10;
    match a {
        // aが0の場合
        0 => {}
        // aが1の場合
        1 => println!("one!!"),
        // aが0, 1以外の全ての場合
        // このパターンは変数名のみで構成され、どんな値にもマッチする
        // マッチした値は新しいローカル変数nに移動またはコピーされる
        // 整数値だけでなく真偽値、文字、文字列も同様
        n => println!("{}!!", n),
    }

    match a {
        100 => println!("100!!"),
        // 全てにマッチするパターンは欲しいが値に興味が無い場合はアンダースコアをパターンとして使う
        // ワイルドカードパターンは全ての値にマッチし、その値をどこにも格納しない
        _ => panic!("panic!!"),
    }
}

fn tuple_structure_pattern() {
    // タプルパターンはtupleにマッチする
    // 複数のデータを一度のmatchで扱いたい場合に使う
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;
        // X座標とY座標それぞれが原点0に一致するか否かをmatchする
        match (x.cmp(&0), y.cmp(&0)) {
            // x, y共に0座標にいた場合
            (Equal, Equal) => "at the origin",
            // yだけ0だった場合 => X軸上にいる
            (_, Equal) => "on the x axis",
            // xだけ0だった場合 => Y軸上にいる
            (Equal, _) => "on the y axis",
            // x, y共に正の座標だった場合 => 第1象限にいる
            (Greater, Greater) => "int the first quadrant",
            // xは負、yは正の座標だった場合 => 第2象限にいる
            (Less, Greater) => "int the second quadrant",
            // 上記以外の全ての場合
            _ => "somewhere else",
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }
    fn structure_pattern(point: Point) {
        // 構造体パターンには構造体式と同様に{}を使う
        // 各フィールドに対するサブパターンも書く
        match point {
            // フィールドxが0の時にマッチするパターン
            Point { x: 0, y: height } => println!("straight up {} meters", height),
            // 上記以外の全てのパターン
            Point { x: x, y: y } => println!("at ({}m, {}m)", x, y),
        }
    }

    // フィールドが沢山ある構造体を例に取る
    struct Account {
        name: String,
        id: i32,
        language: String,
        birthday: String,
        eye_color: String,
    }
    // アカウント情報を取得する関数があるとする
    fn get_account() -> Option<Account> {
        Some(Account {
            name: "myname".to_string(),
            id: 999,
            language: "english".to_string(),
            birthday: "2000/01/01".to_string(),
            eye_color: "blue".to_string(),
        })
    }
    match get_account() {
        // アカウント情報の内、名前と言語以外は使わないと分かっている場合
        // 使わないフィールドへのパターンを全て記述するのは面倒なので..で省略することができる
        // ..で省略した部分に関してはワイルドカードパターンで捨てたのと同じことになる
        Some(Account { name, language, .. }) => println!("{}'s language is {}", name, language),
        _ => println!("error!!"),
    }
}

fn array_slice_pattern() {
    // 配列パターンは配列にマッチする
    // 配列内の位置によって値の意味が変わるような配列において、特別な場合を除去するために使うことが多い
    // 例えばHSLカラー(色相/彩度/輝度)をRGB(赤/緑/青)に変換する際、
    // 輝度が0または255の場合は色相と彩度に関係なくそれぞれ白または黒になる
    fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
        match hsl {
            // 輝度が0の場合は白を返す
            // 色相と彩度は考慮する必要が無いのでワイルドカードパターンで捨てる
            [_, _, 0] => [0, 0, 0],
            // 輝度が255の場合は黒を返す
            [_, _, 255] => [255, 255, 255],
            // それ以外の場合...
            _ => hsl,
        }
    }

    // スライスパターンは配列パターンに似ているが、配列と異なり
    // スライスは可変長であるため値だけでなく長さについてもマッチングを行う
    fn greet_people(names: &[&str]) {
        match names {
            // スライスの長さが0だった場合
            [] => println!("Hello, nobody."),
            // スライスの長さが1だった場合
            [a] => println!("Hello, {}.", a),
            // スライスの長さが2だった場合
            [a, b] => println!("Hello, {} and {}.", a, b),
            // スライスの長さが3以上だった場合
            // スライスパターンでは..を使って任意個の要素とマッチさせることができる
            [a, .., b] => println!("Hello, everyone from {} to {}.", a, b),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
