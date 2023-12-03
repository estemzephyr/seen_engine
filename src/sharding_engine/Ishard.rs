use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::IDataObj::IData::IData;

lazy_static! {
    static ref ID_COUNTER: Mutex<i32> = Mutex::new(0);
}
//Delete After Test
#[derive(Clone,Debug)]
pub struct IShard{
    pub(crate) key:String,
    pub(crate) id:i32,
    pub(crate) ivalue:Vec<IData>,
}
pub struct Shards {
    id: u32,
    key: String,
    shards: Vec<IShard>,
}
pub async fn sharding_process(shard:IShard) {
    let mut shard_vec = Shards::new_shard_vec().await;
    let mut block_counter = 0;
    shard_vec.shards.push(shard);
    for id in 0..=shard_vec.id {
        if id >= 50 {
            shard_vec.id = 0;
            shard_vec.key = format!("Block Number :{}", &block_counter);
            block_counter += 1;
        } else {
            continue;
        }
    }
}

impl Shards {
    pub async fn new_shard_vec() -> Shards {
        let mut id_counter = 0;
        let default_shard = Shards {
            id: id_counter,
            key: String::new(),
            shards: vec![],
        };
        id_counter += 1;
        default_shard
    }
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
}