use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use crate::{Cell, Nonogram};

impl<T: Copy + Serialize> Serialize for Nonogram<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let data: Vec<Vec<Cell<T>>> = self.clone().into();

        data.serialize(serializer)
    }
}

impl<'a, T: Copy + Deserialize<'a>> Deserialize<'a> for Nonogram<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        let data: Vec<Vec<Cell<T>>> = Vec::deserialize(deserializer)?;

        data.try_into()
            .map_err(|_| Error::custom("Failed to construct nonogram."))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let mut src = Nonogram::new(3, 5);
        src[(2, 3)] = Cell::Space;
        src[(1, 0)] = Cell::Box { color: 4 };
        src[(0, 2)] = Cell::Box { color: 2 };

        let json = serde_json::to_string(&src).unwrap();
        let target: Nonogram<i32> = serde_json::from_str(&json).unwrap();

        for col in 0..src.cols() {
            for row in 0..src.rows() {
                assert_eq!(src[(col, row)], target[(col, row)]);
            }
        }
    }
}