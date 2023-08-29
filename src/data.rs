use gloo_storage::{LocalStorage, Storage};
use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub last_position: HashMap<String, usize>,
    pub correct_list: HashSet<String>,
    pub wrong_list: LinkedHashSet<String>,
}

impl Default for Data {
    fn default() -> Self {
        let mut last_position: HashMap<String, usize> = HashMap::new();
        last_position.insert("A".into(), 0);
        last_position.insert("B".into(), 0);
        last_position.insert("C".into(), 0);
        Self {
            last_position: last_position,
            wrong_list: LinkedHashSet::new(),
            correct_list: HashSet::new(),
        }
    }
}

impl Data {
    pub fn get_from_storage() -> Self {
        let data = LocalStorage::get::<Data>("data");
        if let Ok(d) = data {
            return d;
        } else {
            let d = Data::default();
            let _ = LocalStorage::set("data", &d);
            return d;
        }
    }
    pub fn save(&self){
        let _ = LocalStorage::set("data", self);
    }
    pub fn clear(){
        LocalStorage::delete("data");
    }
}
