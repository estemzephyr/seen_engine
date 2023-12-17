use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::L_Data::IDataObj::IData::IData;

lazy_static! {
    static ref ID_COUNTER: Mutex<i32> = Mutex::new(0);
}
//Delete After Test
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct IShard {
    pub(crate) key: String,
    pub(crate) id: i32,
    pub(crate) ivalue: IData,
}

// A function to take first char of data
pub fn take_first_char(data: &String) -> char {
    data.chars().next().unwrap_or('x')
}


impl IShard {
    pub fn default() -> IShard {
        IShard {
            key: String::new(),
            id: 0,
            ivalue: IData::default(),
        }
    }
    pub async fn new_shard(mut self) -> Self {
        let mut counter = ID_COUNTER.lock().unwrap();
        self.id = *counter;
        *counter += 1;
        IShard { key: self.key, id: self.id, ivalue: IData::default() }
    }
}