pub enum ControlProtocol {
    Default,
    Algorithmic,
    MostView,
    Shuffled,
}

impl ControlProtocol {
    pub fn default() -> ControlProtocol {
        //Constructor
        ControlProtocol::Default
    }
    //Sharding engine will be add here and build multithreaded arch
    pub fn process_protocol(self){
            match self{
                ControlProtocol::Algorithmic => {

                }
                ControlProtocol::Default => {
                    println!("Default Protocol is not selection")
                }
                ControlProtocol::MostView => {}
                ControlProtocol::Shuffled => {}
            }
    }
}