use serde_json::Map;
use serde_json::value::Value;
use crate::state::write_file;


pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>)
    {
        state.remove(title);
        write_file("./state.json", state);
        println!("Item {} deleted", title);
    }
}