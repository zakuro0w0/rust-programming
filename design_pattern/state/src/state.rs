/// stateパターン用のインターフェース  
/// 今回はON／OFFのような2つの状態を行き来する状態遷移を想定する  
/// ポリモフィズム(多相)の実現にはgenericsとtraitいずれかの手段が使える  
/// 今回はtraitで実現し、インターフェースをトレイトオブジェクトとして運ぶ  
pub trait State {
    /// switch操作によって次の状態が帰ってくる  
    /// Rust 2015エディションまでは`Box<State>`と記述できた  
    /// Rust 2018エディション以降ではdynキーワードを付ける必要がある  
    /// トレイトオブジェクトとして取り回すものにはdynを付ける  
    fn switch(&self) -> Box<dyn State>;
}

/// スイッチON状態
pub struct On;
/// スイッチOFF状態
pub struct Off;

/// ON状態にStateトレイトを実装する
impl State for On {
    fn switch(&self) -> Box<dyn State> {
        println!("On::switch!!");
        // ON状態の時にスイッチ操作された
        // 次の状態としてOFF状態を返す
        Box::new(Off)
    }
}

/// OFF状態にStateトレイトを実装する
impl State for Off {
    fn switch(&self) -> Box<dyn State> {
        println!("Off::switch!!");
        // OFF状態の時にスイッチ操作された
        // 次の状態としてON状態を返す
        Box::new(On)
    }
}

/// ステートマシンを定義する
pub struct StateMachine {
    /// 現在の状態をStateトレイトオブジェクトとして保持する
    pub current: Box<dyn State>,
}

/// イベントの種類
pub enum Event {
    /// スイッチ操作されたイベント
    Switch,
    /// その他のイベント
    Other,
}

/// ステートマシンに型関連関数を実装する
impl StateMachine {
    /// イベントの発生をステートマシンに知らせる
    /// 現在の状態を書き換えるので`&mut self`で自分自身を受け取る
    pub fn on_event(&mut self, event: Event) {
        // イベントでマッチングを行い、次の状態に更新する
        self.current = match event {
            // スイッチ操作にマッチした場合は現在の状態のswitch()を実行し、次の状態を返す
            Event::Switch => self.current.switch(),
            // その他のイベントにマッチした場合はOFF状態に戻す
            _ => Box::new(Off),
        };
    }
    /// 新しいステートマシンを生成する
    pub fn new() -> StateMachine {
        StateMachine {
            // 初期状態はOFF状態とする
            current: Box::new(Off),
        }
    }
}
