use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::{Cell, Nonogram};

impl<T: Copy + Serialize> Serialize for Nonogram<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let data: Vec<Vec<Cell<T>>> = self.into();

        data.serialize(serializer)
    }
}

impl<'a, T: Copy + Deserialize<'a>> Deserialize<'a> for Nonogram<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        let data: &Vec<Vec<Cell<T>>> = &Vec::deserialize(deserializer)?;

        Ok(data.into())
    }
}