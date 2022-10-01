use crate::observer::trait_object::*;
/// トレイトオブジェクト同士の等価性をチェックするために追加したクレート
/// https://docs.rs/same/latest/same/trait.Same.html
use same::Same;

struct News<'a, String> {
    /// トレイトオブジェクトを格納したBoxへの参照を配列として持つ  
    /// 構造体のフィールドに参照を持つために生存期間パラメータ'aを指定する必要がある
    observers: Vec<&'a O<String>>,
}

/// トレイトのパラメータに生存期間がある場合、implに続けて<>の中で生存期間パラメータを宣言する必要がある  
/// impl, トレイト, トレイトを実装する型 の3つについて生存期間パラメータを明記するので少し面倒に感じる  
impl<'a> Subject<'a, String> for News<'a, String> {
    fn subscribe(&mut self, observer: &'a O<String>) {
        self.observers.push(observer);
        println!("{} subscribed!", observer.name());
    }
    fn unsubscribe(&mut self, observer: &'a O<String>) {
        // sameクレートのsame関数はここで使う
        // Observerリストから引数で指定されたObserverの位置を特定するため、
        // リストの各要素と引数observerを比較する目的でsame()を使う
        // positionのpredicateに渡されるxの型は`&&Box<dyn Observer<String>`になるらしい
        if let Some(index) = self.observers.iter().position(|x| x.same(&observer)) {
            self.observers.remove(index);
            println!("{} unsubscribed!", observer.name());
        }
    }
}

impl<'a> News<'a, String> {
    fn post(&self, article: String) {
        println!("posted new article!");
        for observer in &self.observers {
            observer.notify(&article);
        }
    }
}

#[derive(Clone)]
struct Listener {
    name: String,
}

impl Observer<String> for Listener {
    fn notify(&self, data: &String) {
        println!("{} received subject notify. data=\"{}\"", self.name, data);
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub fn trait_object_pattern() {
    println!("=================== trait object pattern ======================");
    // 購読者: アリス
    let alice: O<String> = Box::new(Listener {
        name: "Alice".to_string(),
    });
    // 購読者: ボブ
    let bob: O<String> = Box::new(Listener {
        name: "Bob".to_string(),
    });
    // ニュース記事
    let mut news = News {
        observers: Vec::new(),
    };
    // ニュース記事の購読開始
    news.subscribe(&alice);
    news.subscribe(&bob);
    // ニュース記事の更新(購読者全員に通知される)
    news.post("today's weather will sunny".to_string());
    // アリスが購読を解除
    news.unsubscribe(&alice);
    // ニュース記事の更新(ボブのみに通知される)
    news.post("tomorrow's weather will raining".to_string());
}
