pub mod structs;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory (item_type:&str, item_title:&str) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        let pending_item = Pending::new (item_title);
        Ok(ItemTypes::Pending(pending_item))
    } else if item_type == "done" {
        let done_item = Done::new (item_title);
        Ok(ItemTypes::Done(done_item))
    } else {
        Err ("This is not accepted")
    }
}