use crate::IDataObj::IData::IData;
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

    pub fn list_shard_with_algorithm(self, mut data: IShard){
        match self {
            ControlProtocol::Alphabetic => {
                println!("{:?}",data.ivalue);

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