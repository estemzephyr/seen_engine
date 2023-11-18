use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ID_COUNTER: Mutex<i32> = Mutex::new(0);
}

pub struct IShard<T>{
    id:i32,
    ivalue:Vec<Option<Box<T>>> // Here using box because we need to keep values on heap
}
impl<T> IShard<T> {
    pub async fn new() -> Self {
        let mut counter = ID_COUNTER.lock().unwrap();
        let id = *counter;
        *counter += 1;
        IShard { id, ivalue: Vec::new() }
    }
}