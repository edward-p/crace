use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use gloo_storage::Storage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data{
    last_position: HashMap<String, usize>,
    wrong_list: Vec<String>,
}

impl Default for Data {
    fn default() -> Self {
        let mut last_position:HashMap<String,usize>=HashMap::new();
        last_position.insert("A".into(), 0);
        last_position.insert("B".into(), 0);
        last_position.insert("C".into(), 0);
        Self { last_position: last_position, wrong_list: Vec::new() }
    }
}
