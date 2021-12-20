use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    pub cols: Vec<Vec<usize>>,
    pub rows: Vec<Vec<usize>>
}

#[derive(Deserialize)]
struct TealLayout {
    ver: Vec<Vec<usize>>,
    hor: Vec<Vec<usize>>
}

impl From<TealLayout> for Layout {
    fn from(teal: TealLayout) -> Self {
        Self {
            cols: teal.hor,
            rows: teal.ver
        }
    }
}

pub fn import(json: &str) -> Result<Layout, serde_json::Error> {
    let layout = serde_json::from_str::<Layout>(json)?;

    Ok(layout)
}

pub fn import_teal(json: &str) -> Result<Layout, serde_json::Error> {
    let layout = serde_json::from_str::<TealLayout>(json)?;

    Ok(layout.into())
}