use std::vec;
use crate::L_Data::IDataObj::IData::IData;
use crate::L_Data::sharding_engine::Ishard::{IShard, take_first_char};

pub enum ControlProtocol {
    Default,
    Alphabetic,
    MostView,
    Shuffled,
}

impl ControlProtocol {
    pub fn default() -> ControlProtocol {
        ControlProtocol::Default
    }

    pub async fn list_shard_with_algorithm_and_insert(self, shards: Vec<IShard>) -> Vec<IShard> {
        match self {
            ControlProtocol::Alphabetic => {
                let mut id_counter = 0;
                let mut _shards: Vec<IShard> = vec![];
                for datas in shards {
                    let first_char = take_first_char(&datas.ivalue.value);
                    let shard_new = IShard{
                        key: format!("Key:{}", first_char),
                        id: 0,
                        ivalue: datas.ivalue,
                    };
                    _shards.push(shard_new);
                    id_counter+=1;
                }
                _shards.sort_by_key(|shard| shard.key.clone());
                _shards
            }
            /*
            ControlProtocol::Default => {
                // Handle the default case here if needed
            }
            ControlProtocol::MostView => {
                // Handle MostView case
            }
            ControlProtocol::Shuffled => {
                // Handle Shuffled case
            }*/
            _ => {
                shards
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_algorithm() {
        let data = IData {
            id: 0,
            name: "Car".to_string(),
            value: "Mustang".to_string(),
        };
        let data1 = IData {
            id: 1,
            name: "Car".to_string(),
            value: "HellCat".to_string(),
        };
        let shard = IShard {
            key: "".to_string(),
            id: 0,
            ivalue: data,
        };
        let shard1 = IShard {
            key: "".to_string(),
            id: 0,
            ivalue: data1,
        };

        let mut shards = vec![];
        shards.push(shard);
        shards.push(shard1);
        let _shards = ControlProtocol::Alphabetic.list_shard_with_algorithm_and_insert(shards)
            .await;

        // Add assertions or println statements to check the results
        for s in _shards {
            println!("{:?}", s);
        }
    }
}
