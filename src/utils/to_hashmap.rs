// pub fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
//     let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
//     let mut map = HashMap::new();
//     for key in keys {
//         let (k, v) = lookup.remove_entry(key).unwrap();
//         map.insert(k, v);
//     }
//     Ok(map)
// }