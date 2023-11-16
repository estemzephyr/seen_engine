use std::fmt::Error;
#[derive(Clone, Debug)]
pub struct IData {
    pub(crate) id: i16,
    pub(crate) name: String,
    pub(crate) i_value: String,
}
impl IData {
    //Constructor to use without "new"
    //Dependency injection
    pub fn default() -> IData {
        IData {
            id: 0,
            name: "".to_string(),
            i_value: "".to_string(),
        }
    }
    pub fn new_data(self) -> Result<IData, Error> {
        let d = IData {
            id: self.id.clone(),
            name: self.name.clone(),
            i_value: self.i_value.clone(),
        };
        Ok(d)
    }
    pub fn get_data(self) -> IData {
        self
    }
    pub fn create_new_data_vec(d:IData)->Vec<IData>{
        let mut vec_new = vec![];
        vec_new.push(d.get_data());
        vec_new
    }
}