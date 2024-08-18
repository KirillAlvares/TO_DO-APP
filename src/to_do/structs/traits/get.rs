use serde_json::Map;
use serde_json::value::Value;


pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>){
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => println!("Item: {}, Status: {}", title, result),
            None => println!("Item {} not found", title),
        }
    }
}