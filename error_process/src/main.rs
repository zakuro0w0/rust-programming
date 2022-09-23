use std::error::Error;
use std::io::{stderr, Write};

enum Weather {
    Sunny,
    Cloud,
    Umbrella,
}

enum Location {
    Japan,
    America,
    Engrand,
}

// Rustには例外が無く、失敗する可能性のある関数は戻り値の型で表現する
// Result<T, E>型はOk(T)かErr(E)のどちらかであり、型T, Eは任意に決定できる
fn get_weather(location: Location) -> Result<Weather, i32> {
    match location {
        Location::Japan => Ok(Weather::Sunny),
        Location::America => Ok(Weather::Umbrella),
        _ => Err(99),
    }
}

fn catch_error() {
    // 次のmatch式はResultを丁寧に処理した場合の実装となるが、少し冗長になる
    match get_weather(Location::Japan) {
        Ok(weather) => {
            println!("Ok!!");
        }
        Err(err) => {
            println!("Err!! {}", err);
        }
    }

    // Result<T, E>型には↑のmatch式相当の処理を簡単に書くためのメソッドがある
    // 成功したか否かをbool値で返す
    let a = get_weather(Location::America).is_ok();
    // エラーだったか否かをbool値で返す
    let b = get_weather(Location::America).is_err();
    // 成功した場合の値をOption<T>として返す
    // 成功していればSome(T)が、失敗していればNoneとなり、エラーの値は捨てられる
    let c = get_weather(Location::America).ok();
    // 失敗した場合の値をOption<E>として返す
    let d = get_weather(Location::America).err();
    // 成功した場合には成功値を返し、失敗した場合にはunwrap_or()に渡したデフォルト値を返す
    // unwrap_or()の戻り値型はOption<T>ではなくTなので.ok()の代わりに使うと便利らしい
    // ただし、適切なデフォルト値が決まっている場面でないと使えない
    let e = get_weather(Location::America).unwrap_or(Weather::Sunny);
    // デフォルト値ではなく、デフォルト値を計算する関数またはクロージャを渡す
    // 結果がエラーだった時のみfallbackが呼び出され、計算されたデフォルト値が返される
    // デフォルト値の計算コストが高い場合に使う
    let f = get_weather(Location::America).unwrap_or_else(|err| Weather::Umbrella);
    // 成功だった場合に成功値を返すが、エラーの場合にはパニックを起こす
    let g = get_weather(Location::America).unwrap();
    // unwrap()とほぼ同じで、パニック発生時のメッセージを指定できる
    let h = get_weather(Location::America).expect("this is panic message");
    // 成功値またはエラー値をResultから借用して、Result<T, E>をResult<&T, &E>に変換する
    // Resultを使い回したい場合に.ok()を呼び出すとResultが消費され、使い回せなくなってしまう
    // このようなケースの時にResult.as_ref().ok()という呼び出しにすると消費されずに済む
    let i = get_weather(Location::America).as_ref();
    // Result<T, E>から可変参照を借用する、戻り値の型はResult<&mut T, &mut E>となる
    let j = get_weather(Location::America).as_mut();
}

fn error_printing(err: std::io::Error) {
    // エラー型にはstd::io::Error, std::fmt::Error, std::str::Utf8Error等色々ある
    // これらは全てstd::error::Errorトレイトを実装していて、以下の機能が使える

    // 全てのエラー型はprintln!マクロで表示することができる
    // フォーマット指定子が{}の場合は短いメッセージが表示される
    // フォーマット指定子を{:?}にするとデバッグ表示となり、技術的情報をより多く得られる
    println!("{}", err);
    // エラーメッセージはto_string()で取り出すこともできる
    let a = err.to_string();
    // エラーの原因となったエラーがある場合にOption型で返す
    // 標準ライブラリには低レベルの機能しか含まれていないため、
    // 標準ライブラリが返すエラーの.source()は多くの場合Noneとなる
    let b = err.source();
    // エラーの値を表示しても原因は表示されない
    // 入手可能な全ての原因を表示したいなら以下のような実装が必要
    fn print_error(mut err: &dyn Error) {
        // writeln!マクロは書き出すストリームを指定でき、ここでは標準エラー出力であるstderrを指定している
        // eprintln!マクロも書き出すストリームを指定できるが、このマクロはエラーが起きるとパニックを起こす
        // このprint_error()ではメッセージを書き出す際のエラーは無視するためにwriteln!マクロを使っている
        let _ = writeln!(stderr(), "error: {}", err);
        while let Some(source) = err.source() {
            let _ = writeln!(stderr(), "caused by: {}", source);
            err = source;
        }
    }
}

fn error_propagation() {
    // let weather = get_weather(Location::Japan)?;
    // ↑の文は↓の文と等価
    // ?演算子は発生したエラーを関数の呼び出し元に伝播させることができる
    // ?演算子はResult<T, E>を生成する任意の式に使うことができる
    // ?演算子は成功した場合にはResultから成功値を取り出して返す
    // ?演算子は失敗した場合には現在の関数を即時returnし、呼び出し元にエラーを渡す
    // このため、?演算子は戻り値型がResultの関数の中でしか使うことができない
    let weather = match get_weather(Location::Japan) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
}

fn main() {
    println!("Hello, world!");
}
