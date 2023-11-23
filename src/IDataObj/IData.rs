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
    pub fn handle_data(self){
        let vec_data = IData::create_new_data_vec(self);
        vec_data.to_vec();
    }
    fn create_new_data_vec(x:IData) ->Vec<IData>{
        let mut vec_new = vec![];
        vec_new.push(x);
        vec_new
    }
}