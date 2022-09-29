/// 生存期間aのObserverトレイト実装型をOとする
pub trait Subject<'a, O: Observer> {
    /// Subjectの更新を購読する
    fn subscribe(&mut self, observer: &'a O);
    /// Subject更新の購読を解除する
    fn unsubscribe(&mut self, observer: &'a O);
}

/// Subjectの購読者==Subscriber
pub trait Observer {
    /// 購読しているSubjectの更新時に実行される
    /// まだ任意のデータを渡すようにはできていない
    fn notify(&self);
}
