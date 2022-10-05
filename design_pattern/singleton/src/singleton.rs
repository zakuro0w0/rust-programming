// 時刻を扱うクレート
use chrono::{DateTime, Local};

/// 今回シングルトン化する構造体  
#[derive(Clone)]
pub struct Something {
    /// 現在日時  
    /// 構造体フィールドをpublicにするとインスタンス化の手段を外部に提供してしまうのでprivateにしておく  
    /// 構造体そのものを公開するよりもトレイトオブジェクトで外部に公開した方が良かったかも
    current_time: DateTime<Local>,
}

impl Something {
    /// 現在日時を出力させる
    pub fn print(self) {
        println!("{}", self.current_time);
    }
}

/// シングルトンなインスタンスを取得する自由関数  
pub fn new() -> Box<Something> {
    // static mutableな変数として唯一のインスタンスを管理する
    // 最初にnew()が実行されるまでは空っぽの状態とするためOption<T>で囲む
    // new()の戻り値とSINGLETONがSomethingをBoxで囲んでいるのは多分ライフタイムの都合
    /**
     * Box<Something>ではなくSomethingを返そうとSINGLETON.unwrap()を
     * 戻り値とした場合は以下のようなコンパイルエラーが出る
     * SomethingにCopyトレイトを実装させれば可能かも
     *
     * cannot move out of static item `SINGLETON`
     * move occurs because `SINGLETON` has type `Option<Something>`, which does not implement the `Copy` traitrustcE0507
     */
    static mut SINGLETON: Option<Box<Something>> = None;
    // SINGLETONはstaticかつmutableなのでunsafeブロックの中でのみアクセス可能
    unsafe {
        // SINGLETONがNone(=未初期化で初回の呼び出しだった)場合はブロックの中身を実行する
        if let None = SINGLETON {
            // SINGLETONに格納する唯一のインスタンスを実体化して保存する
            SINGLETON = Some(Box::new(Something {
                current_time: Local::now(),
            }));
        };
        // SomethingはCopyトレイトを持っていないので明示的にcloneする必要がある
        // clone()したOption<Box<Something>>をunwrap()でOptionから取り出す
        SINGLETON.clone().unwrap()
    }
}
