use std::string::ToString;
use crate::IData::IData::IData;
use crate::sharding_engine::Ishard::IShard;
const ALPHABET:&str = "abcdefghijklmnopqrstuvwxyz";
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

    pub fn list_shard_with_algorithm(self, mut data: IShard<IData>) /*-> Vec<Option<Box<IData>>> */{
        match self {
            ControlProtocol::Alphabetic => {
                for datas in data.ivalue.iter() {
                    println!("{:?}", datas.clone().unwrap().value)
                }
            }
            ControlProtocol::Default => {
                // Handle the default case here if needed
                unimplemented!()
            }
            ControlProtocol::MostView => {
                // Handle MostView case
                unimplemented!()
            }
            ControlProtocol::Shuffled => {
                // Handle Shuffled case
                unimplemented!()
            }
        }
    }
}