use serde_json::Map;
use serde_json::value::Value;
use crate::state::write_file;
use serde_json::json;




pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>){
        state.insert(title.to_string(), json!(String::from ("done")));
        write_file("./state.json", state);
        println!("{} is being set to done", title);
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>){
        state.insert(title.to_string(), json!(String::from ("pending")));
        write_file("./state.json", state);
        println!("{} is being set to pending", title);
    }
}