use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub state: String,
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Item", 2)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("state", &self.state)?;
        state.end()
    }
}
