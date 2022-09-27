/// stateパターン用のインターフェース  
/// 型パラメータTはイベントの種別を指す任意のenum型を想定する
/// ポリモフィズム(多相)の実現にはgenericsとtraitいずれかの手段が使える  
/// 今回はtraitで実現し、インターフェースをトレイトオブジェクトとして運ぶ  
pub trait State<T> {
    /// Rust 2015エディションまでは`Box<State>`と記述できた  
    /// Rust 2018エディション以降ではdynキーワードを付ける必要がある  
    /// トレイトオブジェクトとして取り回すものにはdynを付ける  
    fn on_event(&self, event: T) -> Box<dyn State<T>>;
}

/// ステートマシンを定義する
pub struct StateMachine<T> {
    /// 現在の状態をStateトレイトオブジェクトとして保持する
    pub current: Box<dyn State<T>>,
}

/// ステートマシンに型関連関数を実装する
impl<T> StateMachine<T> {
    /// イベントの発生をステートマシンに知らせる
    /// 現在の状態を書き換えるので`&mut self`で自分自身を受け取る
    pub fn on_event(&mut self, event: T) {
        self.current = self.current.on_event(event);
    }
}
