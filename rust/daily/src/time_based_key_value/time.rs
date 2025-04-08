use std::collections::{HashMap};
struct TimeMap {
    entries: HashMap<String,Vec<(i32,String)>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap{
            entries: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.entries.entry(key).or_default().push((timestamp,value));
    }
    fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(arr) = self.entries.get(&key){
            let index = match arr.binary_search_by(|(t,v)| t.cmp(&timestamp)){
                Ok(i) => i,
                Err(i) => {
                    if i > 0 {
                        i - 1
                    } else {
                        return String::from("");
                    }
                }
            };
            return arr[index].1.clone();
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use crate::time_based_key_value::time::TimeMap;

    #[test]
    fn my_sqrt() {
        let mut t: TimeMap = TimeMap::new();
        // t.set()
    }
}


