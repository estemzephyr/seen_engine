use std::fmt::Error;
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
    pub fn new_data(self) -> Result<IData, Error> {
        let d = IData {
            id: self.id.clone(),
            name: self.name.clone(),
            value: self.value.clone(),
        };
        Ok(d)
    }
    pub fn get_data(self) -> IData {
        self
    }
    pub fn create_new_data_vec()->Vec<IData>{
        let mut vec_new = vec![];
        vec_new
    }
}