use crate::serializable::{Serializable, Serializer};
use crate::debug_serializer::DebugSerializer;
use crate::json_serializer::JsonSerializer;

mod serializable;
mod json_serializer;
mod debug_serializer;

struct User {
    name: String,
    age: u8,
    is_subscribed: bool,
}

impl Serializable for User {
    fn serialize<S: Serializer>(&self, mut serializer: S) -> Result<S::Output, S::Error> {
        serializer.begin_struct("User")?;
        serializer.visit_field("name")?;
        serializer.visit_string(self.name.clone())?;
        serializer.visit_field("age")?;
        serializer.visit_u32(self.age as u32)?;
        serializer.visit_field("is_subscribed")?;
        serializer.visit_bool(self.is_subscribed)?;
        serializer.end_struct()?;
        serializer.finish()
    }
}

fn to_json<T: Serializable>(v: &T) -> Result<String, ()> {
    v.serialize(JsonSerializer::new())
}

fn to_debug<T: Serializable>(v: &T) -> Result<String, ()> {
    v.serialize(DebugSerializer::new())
}

fn main() {
    let p = User { name: "John".to_string(), age: 20, is_subscribed: true };
    let json = to_json(&p).unwrap();
    println!("{}", json);

    let debug = to_debug(&p).unwrap();
    println!("{}", debug);
}
