use std::cmp::Ordering;
use crate::IDataObj::IData::IData;
use crate::sharding_engine::Ishard::{IShard, Shards};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub enum ControlProtocol {
    Default,
    Alphabetic,
    MostView,
    Shuffled,
}

/*impl ControlProtocol {
    pub fn default() -> ControlProtocol {
        ControlProtocol::Default
    }

    pub async fn list_shard_with_algorithm_and_insert(self,shards: Vec<IShard>) -> Shards {
        let mut shard = IShard::default();

        match self {
            ControlProtocol::Alphabetic => {

                //TODO
                // Call the threads for data processing in lifetime to map
                // Overload the memory to take data lifetimes.
                // Process the data's and list with algorithm and send ShardService



                // Memory Loading with threads


                let s = Shards::default();
                Shards::new_shard_vec(s).await
            }

                ControlProtocol::Default => {
                    // Handle the default case here if needed
                    let s = Shards::default();
                    Shards::new_shard_vec(s).await
                }
                ControlProtocol::MostView => {
                    // Handle MostView case
                    let s = Shards::default();
                    Shards::new_shard_vec(s).await
                }
                ControlProtocol::Shuffled => {
                    // Handle Shuffled case
                    let s = Shards::default();
                    Shards::new_shard_vec(s).await
                }
            }
        }
    }
*/
#[cfg(test)]
mod tests {
    use crate::sharding_engine::Ishard::take_first_char;
    use super::*;

    #[tokio::test]
    async fn test_algorithm() {
        let data = IData {
            id: 0,
            name: "Car".to_string(),
            value: "Mustang".to_string(),
        };
        let first_char = take_first_char(vec![]);
        println!("{:?}",first_char);
        let _shards = ControlProtocol::list_shard_with_algorithm_and_insert(ControlProtocol::Alphabetic, data).await;
        println!("{:?}",_shards)
    }
}
