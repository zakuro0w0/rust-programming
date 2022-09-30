use crate::observer::generics::*;

/// 購読対象となる構造体
struct News<'a, O: Observer<String>> {
    /// 購読者リスト
    /// 生存期間aのObserverトレイト実装の参照を配列として持つ
    observers: Vec<&'a O>,
}

/// News構造体へのSubjectトレイトの実装
/// unsubscribe()でObserver同士を比較するためPartialEqトレイトも要求する必要がある
impl<'a, O: Observer<String> + PartialEq> Subject<'a, String, O> for News<'a, O> {
    fn subscribe(&mut self, observer: &'a O) {
        // 購読者をリストに追加する
        self.observers.push(observer);
    }
    fn unsubscribe(&mut self, observer: &'a O) {
        // position()で条件に一致する要素のindexをSome(usize)として取得する
        // 一致する要素が見つかり、if letで指定したパターン(Some(usize))に一致したらブロック内の処理を実行する
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            // 条件に一致する購読者をリストから削除する
            self.observers.remove(index);
        }
    }
}

/// ニュース構造体のメソッド実装
impl<'a, O: Observer<String>> News<'a, O> {
    /// ニュース記事を投稿する
    fn post(&self, article: &String) {
        // 購読者リストの全員に通知を行う
        for observer in &self.observers {
            observer.notify(article);
        }
    }
}

/// 購読者の構造体
#[derive(Debug, PartialEq)]
struct Listener {
    /// 購読者の名前
    name: String,
}

/// Observerトレイトを購読者に実装する
impl Observer<String> for Listener {
    fn notify(&self, data: &String) {
        // 更新の通知が来たら名前を出力する
        println!("{} received subject notify. data=\"{}\"", self.name, data);
    }
}

pub fn generics_pattern_sample() {
    println!("=================== generics pattern ======================");
    // 購読者: アリス
    let alice = Listener {
        name: "Alice".to_string(),
    };
    // 購読者: ボブ
    let bob = Listener {
        name: "Bob".to_string(),
    };
    // ニュース記事
    let mut news = News {
        observers: Vec::new(),
    };
    // ニュース記事の購読開始
    news.subscribe(&alice);
    news.subscribe(&bob);
    // ニュース記事の更新(購読者全員に通知される)
    news.post(&"today's weather will sunny".to_string());
    // アリスが購読を解除
    news.unsubscribe(&alice);
    // ニュース記事の更新(ボブのみに通知される)
    news.post(&"tomollow's weather will raining".to_string());
}
