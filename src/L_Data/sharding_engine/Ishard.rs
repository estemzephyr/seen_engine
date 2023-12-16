use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::L_Data::IDataObj::IData::IData;

lazy_static! {
    static ref ID_COUNTER: Mutex<i32> = Mutex::new(0);
}
//Delete After Test
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct IShard {
    pub(crate) key: String,
    pub(crate) id: i32,
    pub(crate) ivalue: IData,
}

// A function to take first char of data
pub fn take_first_char(data: &String) -> char {
    data.chars().next().unwrap_or('x')
}


impl IShard {
    pub fn default() -> IShard {
        IShard {
            key: String::new(),
            id: 0,
            ivalue: IData::default(),
        }
    }
    pub async fn new_shard(mut self) -> Self {
        let mut counter = ID_COUNTER.lock().unwrap();
        self.id = *counter;
        *counter += 1;
        IShard { key: self.key, id: self.id, ivalue: IData::default() }
    }
}/*
    pub async fn process_shards(data: Vec<IData>) -> Vec<IShard> {
        let mut shards = Vec::new();

        for datas in data {
            let first_char = take_first_char(&datas.value);
            let shard = IShard {
                key: format!("Key_{}", first_char),
                id: 0,
                ivalue: datas.clone(), // IData'nin sahibi olmayan bir kopyasını oluşturuyoruz
            };

            shards.push(shard);
        }
        shards
    }
}

#[cfg(test)]
mod tests {
    use crate::L_Data::IDataObj::IData::IData;
    use crate::L_Data::sharding_engine::Ishard::IShard;

    #[tokio::test]
    async fn test_shard_processor() {
        // Example Data
        let data1 = IData { id: 0, name: "".to_string(), value: "abc".to_string() };
        let data2 = IData { id: 0, name: "".to_string(), value: "def".to_string() };
        let data3 = IData { id: 0, name: "".to_string(), value: "ghi".to_string() };

        // Collection
        let data_vector = vec![data1, data2, data3];

        // Calling function
        let result = IShard::process_shards(data_vector).await;

        // Expected Result
        let expected_output = vec![
            IShard { key: "Key_a".to_string(), id: 0, ivalue: IData { id: 0, name: "".to_string(), value: "abc".to_string() } },
            IShard { key: "Key_d".to_string(), id: 0, ivalue: IData { id: 0, name: "".to_string(), value: "def".to_string() } },
            IShard { key: "Key_g".to_string(), id: 0, ivalue: IData { id: 0, name: "".to_string(), value: "ghi".to_string() } },
        ];

        // Result
        assert_eq!(result, expected_output);

        for shard in &result {
            println!("{:?}", shard);
        }
    }
}
*/