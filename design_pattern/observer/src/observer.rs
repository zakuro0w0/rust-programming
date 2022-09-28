pub trait Subject<T> {
    fn subscribe(&mut self, observer: Box<dyn Observer<T>>);
}

pub trait Observer<T> {
    fn notify(&self, data: &T);
}
