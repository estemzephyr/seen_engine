use crate::sharding_engine::Ishard::IShard;
#[derive(Debug)]
pub struct shard_service {
    shard:IShard,
}
impl shard_service {
    pub async fn ShardEngine() -> shard_service{
        shard_service{
            shard: IShard::default()
        }
    }
}