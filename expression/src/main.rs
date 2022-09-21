enum Types {
    A,
    B,
    C,
}

fn if_expression(x: bool) {
    // Rustのifは文ではなく式として値を返すことができる
    // ifブロックとelseブロックに最後に書いたものが値として返される
    let status = if x { Types::A } else { Types::B };
}

fn match_expression(x: Types) {
    // matchも式として扱われ、値を返すことができる
    // if式とmatch式があるため、RustにはC++等にある三項演算子は無い
    let status = match x {
        // Aなら100が返される
        Types::A => 100,
        // Bなら200が返される
        Types::B => 200,
        // AでもBでもなければ0が返される
        _ => 0,
    };
}

// ブロックによる値の生成
fn block(x: i32) {
    // Rustのブロック{}は値を返す
    let value = {
        // ブロックの中では宣言も可能
        let y = 100;
        let z = x * y;
        // ブロックの最後にセミコロン無しで書いたものが値として返される
        z
    };
}

// ブロック内のアイテムの宣言
// アイテムとはfn, struct, useなどプログラムやモジュールに対してグローバルに現れるもの
fn item_declaration() {
    let value = {
        let a = true;
        // ブロック内部でアイテムを宣言することもできる
        // アイテムは宣言されたスコープ内で使うことができる
        // 同じスコープにあるローカル変数や引数にはアクセスできない
        // 例えば↓のアイテムfoo関数の中からは↑のbool変数aは見えない
        fn foo(x: i32) {
            println!("{}", x);
        }
    };
}

fn if_let_expression(x: Types) {
    // if let式という書き方がある
    // if let pattern = expr { block1 } else { block2 }
    // exprがpatternにマッチする場合にblock1を、それ以外の場合にblock2を実行する
    // match式で特定の1パターンに該当するか否かを調べる場合のエイリアスのような位置づけ
    if let x = Types::A {
        println!("A!!!");
    } else {
        println!("other!!!");
    }

    // 次のmatch式は↑のif let式と等価
    match x {
        Types::A => println!("A!!!"),
        _ => println!("other!!!"),
    }
}

fn main() {
    println!("Hello, world!");
}
