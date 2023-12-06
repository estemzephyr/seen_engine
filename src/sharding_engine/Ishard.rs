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
    //Constructor block
    pub(crate) fn default() -> Shards{
        Shards{
            id: 0,
            key: "".to_string(),
            shards: vec![],
        }
    }
    pub(crate) async fn new_shard_vec(self) -> Shards {
        let mut id_counter = 0;
        let default_shard = Shards {
            id: id_counter,
            key: String::new(),
            shards: self.shards,
        };
        id_counter += 1;
        default_shard
    }
    pub async fn sharding_process(_shard: Vec<IShard>, key: char) {
        //Shadowing for security:Value mutable for working lifetime , after process value isn't mutable.

        //Note: 'key' define in IShardController.rs
        let mut _shard = _shard;
        for mut value in _shard {
            if value.id >= 100{
                value.id = 0;
            }else { continue }
            _shard.push(value)
        }
    }
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