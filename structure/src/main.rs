fn named_field_structure() {
    // 名前付きフィールド型構造体
    // 構造体の名前はアッパーキャメルケースまたはパスカルケース
    struct GrayScaleMap {
        // フィールドやメソッドの名前は小文字のスネークケース
        pixels: Vec<u8>,
        size: (usize, usize),
    }

    let width = 1024;
    let height = 576;
    // 構造体の初期化方法1
    // C++と異なりフィールド名を明示する必要がある
    let image = GrayScaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    // 構造体の初期化方法2
    // フィールド名と同じ名前の変数や引数を並べる方法もある
    fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayScaleMap {
        match true {
            // 変数名とフィールド名が一致している場合、構造体初期化に与える順序は関係無い
            // 同名のフィールドの初期値として使われる
            true => GrayScaleMap { pixels, size },
            false => GrayScaleMap { size, pixels },
        }
    }
    // 構造体インスタンスのフィールドへのアクセスは.で行う
    println!("{}", image.pixels.len());
    println!("{}", image.size.0);

    // 構造体はデフォルトでprivateなので宣言されたモジュールとそのサブモジュールからしか参照できない
    // モジュール外部からも構造体にアクセスしたい場合はpubキーワードを付ける必要がある
    pub struct GrayScaleMap2 {
        // 同様にフィールドもデフォルトでprivateなので、モジュール外部に公開したい場合はpubを付ける
        pub pixels: Vec<u8>,
        pub size: (usize, usize),
    }

    // 構造体のみ公開し、フィールドはモジュール外部に公開しない方法もある
    // 但し、構造体式で初期化するには構造体の全てのフィールドが公開されている必要があるため、
    // GrayScaleMap3をモジュール外部から構造体式で初期化することはできない
    // Vec::new()のようにpublicなインスタンス構築用のメソッドを提供する必要がある
    pub struct GrayScaleMap3 {
        pixels: Vec<u8>,
        size: (usize, usize),
    }
}

// 同じ型の他の変数をベースにして構造体を初期化する方法
fn initialize_structure_omit_expression() {
    // ほうき型モンスターができる活動
    #[derive(Copy, Clone)]
    enum BroomIntent {
        FetchWater,
        DumpWater,
    }
    // ほうき型のモンスターを表現する構造体
    struct Broom {
        name: String,
        height: u32,
        health: u32,
        position: (f32, f32, f32),
        intent: BroomIntent,
    }

    // 1本のほうきを半分に折って2本のほうきを返す関数
    fn chop(b: Broom) -> (Broom, Broom) {
        // 引数のほうきをベースに、長さだけ半分にする
        let mut broom1 = Broom {
            height: b.height / 2,
            // height以外は ..式に与えた変数と同じ値を持たせる
            ..b
        };
        let mut broom2 = Broom {
            // StringはCopyではないので明示的にcloneする必要がある
            name: broom1.name.clone(),
            // name以外は最初に半分に折ったほうきと同じ値で初期化する
            ..broom1
        };
        // 1本目のほうきの名前に I を付ける
        broom1.name.push_str(" I");
        // 2本目のほうきの名前に II を付ける
        broom2.name.push_str(" II");
        // 2本のほうきを返す
        (broom1, broom2)
    }

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::DumpWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey2.height, 30);
}

// タプル型構造体
fn tuple_structure() {
    // tuple型と同じように宣言する
    // tuple型構造体に格納する値はtuple同様に要素(element)と呼ばれる
    struct Bounds(usize, usize);
    // tuple型構造体の初期化はtupleと異なり構造体の名前を与える必要がある
    let image_bounds = Bounds(1024, 768);
    // tuple型構造体の要素へのアクセスはtuple同様に.演算子に番号を与える
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    // tuple型構造体とその要素はそれぞれpublicかprivateにできる
    pub struct Bounds2(pub usize, pub usize);

    // tuple型構造体の使い所1: パターンマッチング
    // 要素アクセスに.演算子を使うことが多い場合はフィールド名でアクセスできた方が可読性が上がる
    // パターンマッチングでアクセスすることが多いならtuple型構造体で十分な場合がある
    let x = match image_bounds {
        // 2番目の要素が768のパターンにマッチする場合
        Bounds(_, 768) => true,
        // 上記以外の場合
        _ => false,
    };

    // tuple型構造体の使い所2: newtypeの定義
    // newtypeとは構成要素が1つだけの構造体、より厳密な型チェックを行うために定義する
    // ASCII文字しか入っていないテキストを扱うためのnewtypeは以下のように定義できる
    // Vec<u8>のまま運搬するよりも意図が伝わりやすくなる(C++のusingによる型エイリアスのようなものだろうか)
    struct Ascii(Vec<u8>);
}

fn unit_structure() {
    // ユニット型構造体
    // このような型の値はユニット型()と同様にメモリを消費しない
    // ユニット型構造体はトレイトを扱う際にも役立つらしい
    struct Onesuch;
    // 1つの状態しか取り得ない型となっている
    let a = Onesuch;
}

// FIFOキューの実装
pub struct Queue {
    // 古い要素群、最も古いものが末尾
    older: Vec<char>,
    // 新しい要素群、最も新しいものが末尾
    younger: Vec<char>,
}
// Rustの構造体のメソッドは構造体の外でimplブロックに定義する
// implブロックに書いた関数は特定の型に関連付けられているため、関連関数(associated function)と呼ばれる
// implブロックの外で定義される関数はこれに対して自由関数(free function)と呼ばれる
impl Queue {
    // Rustはメソッドに対して呼び出される対象となる値を最初の引数として与える
    // この第1引数は特別な名前selfでなければならない
    // selfの型はimplブロックで指定した型あるいはその型への参照なので型定義は省略できる
    // self: Queue, self: &Queue, self: &mut Queueではなく、
    // self, &self, &mut selfのように省略形で書くことができる
    pub fn push(&mut self, c: char) {
        // Queue構造体のメンバへのアクセスにはselfを経由する必要がある
        self.younger.push(c);
    }
    // Queue::push, Queue::popはどちらもQueue構造体を変更するため、&mut selfを引数に取る
    // しかしメソッド呼び出し時に可変参照を借用する必要は無く、メソッド呼び出し構文が暗黙にやってくれる
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
    // 構造体を変更しないメソッドであれば自身の共有参照を受け取るように定義する
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    // メソッドがselfの所有権を取得したい場合はselfを値で受け取ることもできる
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

fn impl_method_declaration() {
    let mut queue = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    queue.push('0');
    queue.push('1');
    assert_eq!(queue.pop(), Some('0'));
    queue.push('∞');
    assert_eq!(queue.pop(), Some('1'));
    assert_eq!(queue.pop(), Some('∞'));
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.is_empty(), true);

    // 構造体の所有権を取得するメソッドの呼び出し
    queue.push('x');
    queue.push('y');
    let (older, younger) = queue.split();
    assert_eq!(older, vec!['x']);
    assert_eq!(younger, vec!['y']);
    // 先のQueue::split()呼び出しでqueueの所有権は失われた
    // ↓のコードはコンパイルエラーになる
    // let a = queue.is_empty();
}

fn type_associated_function() {
    // 特定の型に対するimplブロックではselfを引数に取らない関数も定義できる
    // これはselfを引数に取らないのでメソッドではなく、型関連関数(type associated function)と呼ばれる
    // C++クラスのstatic関数とよく似ている
    // 1つの型に対して複数のimplブロックを書くこともできるが、
    // 全てのimplブロックは型を定義したクレートに含まれている必要がある
    impl Queue {
        // 型関連関数はコンストラクタを提供するためによく使われる
        pub fn new() -> Queue {
            Queue {
                older: Vec::new(),
                younger: Vec::new(),
            }
        }
    }
    // 型が持つメソッドを型定義と別のimplブロックに書くスタイルには以下のようなメリットがある
    // 1. 型のデータメンバを見つけるのが簡単になる(C++ではクラス全て読まないと見逃す可能性がある)
    // 2. タプル型やユニット型の構造体にメソッドを綺麗に入れるのが難しいが、implブロックにより1つの構文にまとまる
    // 3. 同じimpl構文をトレイトの実装にも使用できる

    let mut queue = Queue::new();
    queue.push('*');
}

// 型関連定数
fn type_associated_constant() {
    pub struct Vector2 {
        x: f32,
        y: f32,
    }
    // 型の特定のインスタンスではなく型そのものに値を関連付けるのが型関連定数(associated const)
    impl Vector2 {
        // constを付ける、大文字で区切りはアンダースコアを使う
        const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
        const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
        const NAME: &'static str = "Vector2";
        const ID: u32 = 18;
    }
    // 型関連定数へのアクセスは型::定数名
    let x = Vector2::ID;
}

fn generic_structure() {
    // 型パラメータTで任意の型を表現できる
    // TにはcharやStringなど任意の型を指定できる
    pub struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>,
    }
    // 任意の型Tに対してQueue<T>で利用できる型関連関数を定義する
    impl<T> Queue<T> {
        // newの戻り値型はQueue<T>だが、これはSelfと書くこともできる
        // genericであるか否かに関係無く、全てのimplブロックでは特別な型パラメータSelfが参照可能
        // Selfはimplブロックで指定した型を指すことになる
        pub fn new() -> Self {
            Queue {
                older: Vec::new(),
                younger: Vec::new(),
            }
        }
        pub fn push(&mut self, t: T) {
            self.younger.push(t);
        }
        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
    }

    // ↓はT=f64の時だけ利用できる型関連関数を実装する
    impl Queue<f64> {
        // sum()はQueue<f64>に対してのみ使用可能
        fn sum(&self) -> f64 {
            99.9
        }
    }

    // ↓の書き方はコンパイルエラーになる
    // let mut a = Queue<char>::new();
    // ↓のようにターボフィッシュ(::<)を使って明示する書き方が必要
    let mut a = Queue::<char>::new();
    // 実際には型パラメータTを明示することは少なく、Rustに推論させる使い方が多い
    let mut b = Queue::new();
    let mut c = Queue::new();
    // ↓から明らかにbはQueue<&'static str>と推論される
    b.push("ABC");
    // ↓から明らかにcはQueue<f64>と推論される
    c.push(0.99);
}

fn main() {
    println!("Hello, world!");
    initialize_structure_omit_expression();
}
