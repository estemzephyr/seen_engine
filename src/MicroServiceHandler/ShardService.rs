use async_trait::async_trait;
use crate::L_Data::sharding_engine::Ishard::IShard;
use crate::L_Data::sharding_engine::IShardController::ControlProtocol;
use crate::L_Data::sharding_engine::shard_manager::shard_service;

#[async_trait]
pub trait ShardServiceEngine{
    async fn list_with_algorithm(self,shards:Vec<IShard>) -> Vec<IShard>;

}
#[async_trait]
impl ShardServiceEngine for shard_service{
    async fn list_with_algorithm(self, shards: Vec<IShard>) -> Vec<IShard>{
       ControlProtocol::list_shard_with_algorithm_and_insert(ControlProtocol::Alphabetic,shards).await
    }
}