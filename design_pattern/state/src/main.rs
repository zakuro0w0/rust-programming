// stateモジュールがstate.rsに記述されていることをコンパイラに知らせる
// ↓の1行だけでstate.rsの中身は`mod state{}`で囲まれたのと同じ状態になり、
// コンパイル対象に含まれるようになる
mod state;
// 自分のcrateのstateモジュール配下全てについて使用宣言する
use crate::state::*;

/// スイッチON状態
pub struct On;
/// スイッチOFF状態
pub struct Off;

/// イベントの種類
pub enum Event {
    /// スイッチ操作されたイベント
    Switch,
    /// その他のイベント
    Other,
}

/// ON状態にStateトレイトを実装する
impl State<Event> for On {
    fn on_event(&self, event: Event) -> Box<dyn State<Event>> {
        println!("On::switch!!");
        // ON状態の時にスイッチ操作された
        // 次の状態としてOFF状態を返す
        Box::new(Off)
    }
}

/// OFF状態にStateトレイトを実装する
impl State<Event> for Off {
    fn on_event(&self, event: Event) -> Box<dyn State<Event>> {
        println!("Off::switch!!");
        // OFF状態の時にスイッチ操作された
        // 次の状態としてON状態を返す
        Box::new(On)
    }
}

fn main() {
    println!("Hello, world!");
    // 新しいステートマシンを生成(状態が遷移するのでmutableで作る)
    let mut state_machine = state::StateMachine {
        current: Box::new(Off),
    };
    // イベントの発生をステートマシンに通知する度にマシン内部で状態遷移が起こる
    state_machine.on_event(Event::Switch);
    state_machine.on_event(Event::Switch);
    state_machine.on_event(Event::Switch);
}
