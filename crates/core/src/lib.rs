use slotmap::{new_key_type, SlotMap};
use std::collections::HashMap;

new_key_type! {
    pub struct LayerId;
    pub struct ObjectId;
}

#[derive(Default)]
pub struct Workplan {
    pub layers: SlotMap<LayerId, Layer>,
}

#[derive(Default)]
pub struct Layer {
    pub objects: SlotMap<ObjectId, Object>,
    pub tiles: HashMap<(i32, i32), Tile>,
}

#[derive(Default)]
pub struct Object {}

#[derive(Default)]
pub struct Tile {}
