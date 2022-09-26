// stateモジュールがstate.rsに記述されていることをコンパイラに知らせる
// ↓の1行だけでstate.rsの中身は`mod state{}`で囲まれたのと同じ状態になり、
// コンパイル対象に含まれるようになる
mod state;
// 自分のcrateのstateモジュール配下全てについて使用宣言する
use crate::state::*;

fn main() {
    println!("Hello, world!");
    // 新しいステートマシンを生成(状態が遷移するのでmutableで作る)
    let mut state_machine = state::StateMachine::new();
    // イベントの発生をステートマシンに通知する度にマシン内部で状態遷移が起こる
    state_machine.on_event(state::Event::Switch);
    state_machine.on_event(state::Event::Switch);
    state_machine.on_event(state::Event::Switch);
}
