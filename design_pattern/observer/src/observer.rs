pub mod generics {
    /// 生存期間aのObserverトレイト実装型をOとする
    pub trait Subject<'a, T, O: Observer<T>> {
        /// Subjectの更新を購読する
        fn subscribe(&mut self, observer: &'a O);
        /// Subject更新の購読を解除する
        fn unsubscribe(&mut self, observer: &'a O);
    }

    /// Subjectの購読者==Subscriber
    pub trait Observer<T> {
        /// 購読しているSubjectの更新時に実行される
        /// 任意の型Tのデータを通知できる
        fn notify(&self, data: &T);
    }
}

pub mod trait_object {
    pub trait Subject<'a, T> {
        fn subscribe(&mut self, observer: &'a Box<dyn Observer<T>>);
        fn unsubscribe(&mut self, observer: &'a Box<dyn Observer<T>>);
    }
    pub trait Observer<T> {
        fn notify(&self, data: &T);
    }
}
