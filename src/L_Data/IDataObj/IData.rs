use std::fmt::Display;


#[derive(PartialEq, Clone, Debug)]
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
            name: String::new(),
            value: String::new(),
        }
    }
    pub fn create_new_data_vec() -> Vec<IData> {
        let mut vec_new: Vec<IData> = vec![];
        vec_new
    }
    pub async fn get_datas_on_vec(x: Vec<IData>) -> IData {
        let mut data = IData::default();
        for value in x {
            data = value;
        }
        data
    }

    pub async fn clean_up_values(&mut self) {
        self.name = self.name.trim_matches('"').to_string();
        self.value = self.value.trim_matches('"').to_string();
    }
}
