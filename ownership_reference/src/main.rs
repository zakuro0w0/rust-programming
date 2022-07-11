fn ownership1() {
    let a = "aaaaaa".to_string();
    // 代入は所有権の移動(move)を引き起こす
    // 文字列の所有権はaからbに移り、aはドロップされるためアクセスできなくなる
    let b = a;

    // error[E0382]: borrow of moved value: `a`
    //  --> src/main.rs:4:20
    //   |
    // 2 |     let a = "aaaaaa".to_string();
    //   |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
    // 3 |     let b = a;
    //   |             - value moved here
    // 4 |     println!("{}", a);
    //   |                    ^ value borrowed here after move
    //   |
    //   = note: this error originates in the macro `$crate::format_args_nl`
    //   (in Nightly builds, run with -Z macro-backtrace for more info)

    // このコードは↑のコンパイルエラーを引き起こす
    // println!("{}", a);
}

fn ownership2() {
    // Vec<String>を作る
    let a = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
    // 単純な代入では所有権のmoveになるため、配列のコピーを作る場合はclone()を使う
    // clone()はディープコピーによりVecとその要素の完全なコピーを作る
    // このためb, cはaの所有権とは独立した存在になる
    // clone()によるコピーは実体が増えるためコストが高いことに留意する
    let b = a.clone();
    let c = a.clone();
}

fn f(x: String) {
    println!("{}", x);
}

fn ownership3() {
    let a = "aaaa".to_string();
    f(a);
    // ↓のf()呼び出しはコンパイルエラーになる
    // 関数に引数として入力するコードも変数の所有権移動を伴うため
    // f(a);
}

fn g(x: &String) {
    println!("{}", x);
}

fn reference() {
    let a = "aaaa".to_string();
    g(&a);
    // ownership3()と異なり、↓のg()呼び出しはコンパイルエラーにならない
    // g()の引数型がStringではなく&Stringという共有参照になっており、
    // 所有権の移動を伴わず借用という形で共有しているためである
    // 借用中(=g()のスコープが有効な間)は所有者であっても変数aを変更できない
    g(&a);
}

fn main() {
    println!("Hello, world!");
}
