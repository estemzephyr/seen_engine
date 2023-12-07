use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::IDataObj::IData::IData;

lazy_static! {
    static ref ID_COUNTER: Mutex<i32> = Mutex::new(0);
}
//Delete After Test
#[derive(Clone, Debug)]
pub struct IShard {
    pub(crate) key: String,
    pub(crate) id: i32,
    pub(crate) ivalue: Vec<IData>,
}

#[derive(Debug)]
pub struct Shards {
    id: u32,
    key: String,
    shards: Vec<IShard>,
}


impl Shards {
    pub(crate) async fn new_shard_vec(self) -> Shards {
        let mut id_counter = 0;
        let default_shard = Shards {
            id: id_counter,
            key: "".to_string(),
            shards: self.shards,
        };
        id_counter += 1;
        default_shard
    }
    pub async fn sharding_process(_shard: Vec<IShard>, key: char) {
        let mut id_counter = 0;
        let mut shard_vec = Shards {
            id: id_counter,
            key: "".to_string(),
            shards: vec![],
        };
        for mut value in _shard {
            if value.id >= 100 {
                value.id = 0;
                value.key = String::from(take_first_char(value.ivalue))
            }
        }
    }
}
// A function to take first char of data
pub fn take_first_char(data: Vec<IData>) -> char {
    data.iter().flat_map(|shard| shard.value.chars().next()).next().unwrap_or('x')
}


impl IShard {
    pub fn default() -> IShard {
        IShard {
            key: String::new(),
            id: 0,
            ivalue: vec![],
        }
    }
    pub async fn new_shard(mut self) -> Self {
        let mut counter = ID_COUNTER.lock().unwrap();
        self.id = *counter;
        *counter += 1;
        IShard { key: self.key, id: self.id, ivalue: Vec::new() }
    }
}