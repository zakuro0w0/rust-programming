mod observer;
use observer::*;

struct News<T> {
    observers: Vec<Box<dyn Observer<T>>>,
}

impl<T> Subject<T> for News<T> {
    fn subscribe(&mut self, observer: Box<dyn Observer<T>>) {
        self.observers.push(observer);
    }
}

impl<T> News<T> {
    fn post(&self, data: &T) {
        for observer in &self.observers {
            observer.notify(data);
        }
    }
}

struct Listener {}

impl<T> Observer<T> for Listener {
    fn notify(&self, data: &T) {
        println!("new data notified!!");
    }
}

fn main() {
    println!("Hello, world!");
    let listener = Listener {};
    let mut news = News::<String> {
        observers: Vec::new(),
    };
    news.subscribe(Box::new(listener));
    news.post(&"todays weather news!!".to_string());
}
