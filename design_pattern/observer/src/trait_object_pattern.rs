use crate::observer::trait_object::*;

struct News<'a, String> {
    observers: Vec<&'a Box<dyn Observer<String>>>,
}

impl<'a> Subject<'a, String> for News<'a, String> {
    fn subscribe(&mut self, observer: &'a Box<dyn Observer<String>>) {
        self.observers.push(observer);
    }
    fn unsubscribe(&mut self, observer: &'a Box<dyn Observer<String>>) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
        }
    }
}

impl<'a> News<'a, String> {
    fn post(&self, article: String) {
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
}

pub fn trait_object_pattern() {
    println!("=================== trait object pattern ======================");
    // 購読者: アリス
    let alice: Box<dyn Observer<String>> = Box::new(Listener {
        name: "Alice".to_string(),
    });
    // 購読者: ボブ
    let bob: Box<dyn Observer<String>> = Box::new(Listener {
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
    news.post("tomollow's weather will raining".to_string());
}
