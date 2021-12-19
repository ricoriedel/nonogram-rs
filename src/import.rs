use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::algo;

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    pub cols: Vec<Vec<usize>>,
    pub rows: Vec<Vec<usize>>
}

#[derive(Deserialize)]
pub struct TealLayout {
    ver: Vec<Vec<usize>>,
    hor: Vec<Vec<usize>>
}

impl Into<Layout> for TealLayout {
    fn into(self) -> Layout {
        Layout {
            cols: self.hor,
            rows: self.ver
        }
    }
}