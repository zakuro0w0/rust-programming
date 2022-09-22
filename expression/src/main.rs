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

fn loop_expression(x: bool) {
    // 一般的なwhileループ
    // whileに続くconditionがtrueの間ブロック内の処理が繰り返される
    // Rustのwhileもifやmatchと同様に式であり、返す値は常に()となる
    while x {
        println!("this is while loop");
    }

    // if let式のwhileバージョン
    // 1周する毎にpatternとexprが一致するかを検証する
    while let x = false {
        println!("this is while let loop");
    }

    // 範囲for文
    // inに続くiterableの要素を1個ずつ取り出して処理する
    for a in 0..10 {
        println!("{}", a);
    }

    // 無限ループを書くための構文
    // breakかreturnに到達する、あるいはスレッドがパニックを起こすまで継続
    loop {
        println!("this is loop");
    }
}

fn bad_for_expression() {
    let strings = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
    // 個々のStringはstringsからsに移動する
    // 値をforループで処理するとRustの移動セマンティクスに従って値が消費されてしまう
    for s in strings {
        println!("{}", s);
    } // 移動したsはここでドロップされる
      // forのスコープを抜けた後、stringsには要素は残っていない
      // このため↓の文はコンパイルエラーになる
      // println!("{}", strings.len());
}

fn good_for_expression() {
    let strings = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
    // コレクションに対してループするのではなく、コレクションの参照に対してループすればOK
    for rs in &strings {
        // ループ変数rsはコレクションの要素に対する参照となる
        // &stringsの型は&Vec<String>, rsの型は&Stringになる
        println!("{}", rs);
    }

    let mut strings2 = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
    for rs in &mut strings2 {
        // mutableな参照に対するループでは、各要素に対するmut参照が作られる
        println!("{}", rs);
    }
}

fn break_expression(x: bool) {
    // loopのボディ部でbreakに与えた値がloop式の返す値となる
    // 当然ながら、break式が返す値の型は全てのbreak式で一致している必要がある
    let answer = loop {
        if x {
            break "x is true";
        } else {
            break "x is false";
        }
    };

    let list: Vec<i32> = (0..10).collect();
    for i in list {
        if i % 2 == 0 {
            println!("{}", i);
        } else {
            // continueで次のループに飛ぶのはC++と同じ
            continue;
        }
    }

    // 数列の次の数を返すことにした関数
    fn next_number() -> i32 {
        10
    }

    // ループには'outerのような任意のラベルを付けることができる
    // breakで指定したラベルのループを一気に中断させることができる
    let sqrt = 'outer: loop {
        let n = next_number();
        for i in 1.. {
            let square = i * i;
            if square == n {
                // breakにはラベルと値の両方を一度に指定できる
                // 外側のループを中断しつつ、値iをloop式の値として返す
                break 'outer i;
            }
            if square > n {
                // ラベルを指定しない場合はC++と同様最も近いループを中断する
                // ラベルはcontinueにも使うことができる
                break;
            }
        }
    };
}

fn turbo_fish_expression() {
    /**
     * 次の文は以下のコンパイルエラーを引き起こす
     * '<'が「小なり」の比較演算子であることに由来しており、
     * コンパイラは'<T>'ではなく'::<T>'を使うようにヒントを提示する
     * このシンボル '::<'はRustコミュニティではターボフィッシュと呼ばれている
     *
     * comparison operators cannot be chainedrustc
     * main.rs(173, 18): use `::<...>` instead of `<...>` to specify type or const arguments: `::`
     */
    // let vec = Vec<i32>::with_capacity(1000);
    // ターボフィッシュを使えばOK
    let vec = Vec::<i32>::with_capacity(1000);

    fn bar() -> Vec<i32> {
        // 型パラメータ<T>の記述を省略し、コンパイラに推論させる書き方もできる
        return Vec::with_capacity(1000);
    }
}

fn range_expression() {
    // RangeFull
    let a = ..;
    // RangeFrom<i32>
    let b = 0..;
    // cとdは末尾を含まないhalf-open(半開区間)となる
    // RangeTo<i32>
    let c = ..10;
    // Range<i32>
    let d = 0..10;
    // eとfは..=演算子により末尾を含むclosed(閉区間)となる
    // RangeToInclusive<i32>
    let e = ..=10;
    // RangeInclusive<i32>
    let f = 0..=10;

    // 繰り返し処理に使えるのは開始値を含む範囲のみ
    // ↑の変数で言えばb, d, fのみ
}

fn closure_expression() {
    // ラムダ式に相当するクロージャの書き方
    let is_even = |x| x % 2 == 0;
    // 引数xの型は呼び出し方から推論される
    is_even(10);
    // ↑のis_evenの型をちゃんと書くと↓のようになる
    // クロージャの戻り値型を明示した場合はボディ部分をブロックで囲む必要がある
    let is_even2 = |x: i32| -> bool { x % 2 == 0 };
}

fn main() {
    println!("Hello, world!");
}
