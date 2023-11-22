use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::IDataObj::IData::IData;

lazy_static! {
    static ref ID_COUNTER: Mutex<i32> = Mutex::new(0);
}
//Delete After Test
#[derive(Clone)]
pub struct IShard{
    pub(crate) key:String,
    pub(crate) id:i32,
    pub(crate) ivalue:Vec<IData>,
}

impl IShard {
    pub fn default() -> IShard {
        IShard{
            key: String::new(),
            id: 0,
            ivalue: vec![],
        }
    }
    pub async fn new_shard(mut self) -> Self {
        let mut counter = ID_COUNTER.lock().unwrap();
        self.id = *counter;
        *counter += 1;
        IShard {key: self.key, id:self.id, ivalue: Vec::new() }
    }
    pub fn get_shards(self) -> IShard {
        self
    }
}