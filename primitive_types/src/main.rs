fn integer_types() {
    // 基本的には推論してもらえる
    let a = 10;
    // 型を指定する方法もある
    let b: i32 = 20;
    // 数値の後ろに型を書く方法もある
    let c = 30i32;
    // 数値には任意の位置に`_`による区切りを挿入できる
    let d = 30_i32;
    // 符号なし整数はuを付ける
    let e: u32 = 40;
    // 計算機アドレスと同じサイズの整数はisize, usizeで表現する
    let f: isize = 50;
    let g: usize = 60;
}

fn float_types() {
    // 浮動小数点型の表現はf32, f64
    let a: f32 = 10.5;
    let b = 0.99999999_f64;
}

fn boolean_types() {
    // 真偽値型はbool
    // trueかfalseのどちらか
    let a: bool = true;
    let b = false;
    if a {
        println!("a={}", a);
    }
}

fn char_types() {
    // 文字型はシングルクォートで囲む
    let a: char = '\n';
    let b = '*';
    let c = '字';
    let d = '\u{CA0}';
    // Rustのcharは整数型にはならない(↓のコードはコンパイル時エラーになる)
    // let e: i32 = 'A';
}

fn tuple_types() {
    // 任意の型を任意の数でセットにできる
    // 配列とは違って異なる型を混ぜてもOK
    let a: (i32, char, bool) = (10, 'X', false);
    // 空のtupleはユニット型として記述される
    // Result<(), std::io::Err>のような成功時に何も返さないResult型の記述などで必要になる
    let b: () = ();
}

fn struct_types() {
    // 構造体の宣言
    struct T {
        x: bool,
        y: char,
        z: u32,
    }
    // 構造体のインスタンス化(C++と異なりフィールドの名前が必須)
    let a = T {
        x: true,
        y: 'A',
        z: 100,
    };
    // 構造体インスタンスのフィールドへのアクセス
    println!("{}", a.x);

    // フィールドを持たないユニット型構造体
    struct E;
    let b = E;
}

fn enum_types() {
    // 列挙型
    enum Types {
        A,
        B,
        C,
        // 代数データ型と書いてあったが用途がよく分からない
        D(u32),
    }
    // 列挙型フィールドへのアクセスはC++と同じ
    let a = Types::A;
    let b = Types::D(99);
}

fn box_types() {
    enum E {
        A,
        B,
        C,
    }
    // Boxで囲むとヒープ領域の値へのポインタが保持できるらしい
    let a = Box::new(E::A);
}

fn reference_types() {
    // 普通のi32型変数, aは変更できない
    let a: i32 = 10;
    // i32型への共有参照, bも変更できない
    let b: &i32 = &a;

    // mutableなi32型変数, cは変更できる
    let mut c: i32 = 20;
    // i32型への可変参照, dも変更できる
    let d: &mut i32 = &mut c;
}

fn string_types() {
    // UTF-8の文字列、サイズは動的に変化する
    let a: String = "abcdefghijklmn".to_string();
    // 文字列への参照(UTF-8テキストへの所有権の無い参照)
    let b: &str = "bbbbb";
    let c: &str = &a[0..5];
}

fn array_types() {
    // f64型の要素が4個の固定長配列(もちろん型の宣言無しでも推論してくれる)
    let a: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    // 256個の要素全てをfalseで初期化した固定長配列
    let b: [bool; 256] = [false; 256];

    // i32型の可変長配列の初期化
    let mut c: Vec<i32> = vec![10, 20, 30];
    // mutableであれば要素の追加も可能
    c.push(40);
    c.push(50);

    // Vec::new()でも初期化できる
    let mut d = Vec::new();
    d.push("dog");
    d.push("cat");

    // イテレータで生成する値をcollectする初期化方法
    // collect()する場合は受け取る側の型の明示が必要
    let e: Vec<i32> = (0..10).collect();
    // もちろん配列はforeachできる
    for x in &e {
        println!("{}", x);
    }
    // 配列のサイズはlen()で取得できる
    println!("size={}", &e.len());
}

fn option_types() {
    // 失敗した場合のNoneまたは成功した場合のSome<T>を保持できるOption型
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;
    // match式で成否判定などに使える
    let c = match a {
        Some(v) => v,
        None => {
            eprintln!("Error!!!");
            -100
        }
    };
}

fn main() {
    println!("Hello, world!");
}
