#[derive(Clone, Debug)]
pub struct IData {
    pub(crate) id: i16,
    pub(crate) name: String,
    pub(crate) value: String,
}
impl IData {
    //Constructor
    pub fn default() -> IData {
        IData {
            id: 0,
            name: "".to_string(),
            value: "".to_string(),
        }

    }
    // Setting the values for data implements self , not referring bcz value will drop after using
    pub fn create_new_data(x:IData) -> IData {
        let life_cycle_data=IData::create_new_data(x);
        life_cycle_data
    }
    fn create_new_data_vec(x:IData) ->Vec<IData>{
        let mut vec_new = vec![];
        vec_new.push(x);
        vec_new
    }
}