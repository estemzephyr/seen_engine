use crate::sharding_engine::Ishard::IShard;

pub struct Shards {
    id: u32,
    key: String,
    shards: Vec<IShard>,
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

pub struct shard_service {
    i_shard: IShard,
}

impl shard_service {
    pub async fn ShardEngine(i_shard: IShard) {
        let mut shard_vec = Shards::new_shard_vec().await;
        let mut block_counter = 0;
        shard_vec.shards.push(i_shard);
        for id in shard_vec.id {
            if id >= 50 {
                shard_vec.id = 0;
                shard_vec.key = format!("Block Number :{}", &block_counter);
                block_counter += 1;
            } else {
                continue;
            }
        }
    }
}