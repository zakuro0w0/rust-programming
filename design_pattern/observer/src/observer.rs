/// ジェネリクス構文で多相を実現したパターン
pub mod generics {
    /// 生存期間aのObserverトレイト実装型をOとする  
    /// subscribe(), unsubscribe()はdyn Traitのトレイトオブジェクトではなく、  
    /// 任意の型Tを扱うObserverトレイトを実装した生存期間aの任意の実体型をobserver引数として受け取る
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
        fn name(&self) -> &str;
    }
}

/// トレイトオブジェクト構文で多相を実現したパターン
pub mod trait_object {

    /// トレイトオブジェクトを運搬するBoxの型エイリアス
    pub type O<T> = Box<dyn Observer<T>>;
    pub trait Subject<'a, T> {
        /// トレイトオブジェクトは基本的にBoxで囲んで運ぶ必要がある  
        /// Boxで囲んでいても参照でない形で渡すと所有権を呼び出し元から奪ってしまう  
        /// Observerはsubscribeとunsubscribeで複数回使うので所有権を奪っては都合が悪い  
        /// このためトレイトオブジェクトもBoxの参照を渡す必要があり、  
        /// この参照への配列をSubject実装型で持つので生存期間パラメータ'aも必要になる  
        fn subscribe(&mut self, observer: &'a O<T>);
        fn unsubscribe(&mut self, observer: &'a O<T>);
    }
    pub trait Observer<T> {
        fn notify(&self, data: &T);
        fn name(&self) -> &str;
    }
}
