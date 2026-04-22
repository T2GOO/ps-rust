use slotmap::{new_key_type, SlotMap};
use std::collections::HashMap;

new_key_type! {
    pub struct LayerId;
    pub struct ObjectId;
}

pub const TILE_SIZE: u32 = 64;
pub type TileKey = (i32, i32);

pub struct Tile {
    pub pixels: Vec<[u8; 4]>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    #[default]
    Normal,
}

#[derive(Clone, Debug)]
pub enum EffectKind {
    Placeholder,
}

pub enum Object {
    Image {
        tiles: HashMap<TileKey, Tile>,
        blend: BlendMode,
        opacity: f32,
    },
    Effect {
        kind: EffectKind,
        density: Vec<u8>,
    },
}

pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
}

#[derive(Default)]
pub struct Selection {
    pub active_layer: Option<LayerId>,
    pub active_object: Option<ObjectId>,
}

pub struct TileSnapshot {
    pub object_id: ObjectId,
    pub key: TileKey,
    pub before: Vec<[u8; 4]>,
    pub after: Vec<[u8; 4]>,
}

#[derive(Default)]
pub struct History {
    pub past: Vec<Vec<TileSnapshot>>,
    pub future: Vec<Vec<TileSnapshot>>,
}

#[derive(Default)]
pub struct Viewport {
    pub offset: (f32, f32),
    pub zoom: f32,
}

#[derive(Default)]
pub struct AppState {
    pub layers: SlotMap<LayerId, Layer>,
    pub objects: SlotMap<ObjectId, Object>,
    pub layer_order: Vec<LayerId>,
    pub layer_objects: HashMap<LayerId, Vec<ObjectId>>,
    pub selection: Selection,
    pub history: History,
    pub viewport: Viewport,
}

pub enum UiCmd {
    AddLayer {
        name: String,
    },
    RemoveLayer {
        id: LayerId,
    },
    MoveLayer {
        id: LayerId,
        new_index: usize,
    },
    AddImageObject {
        layer: LayerId,
    },
    PaintTile {
        object_id: ObjectId,
        key: TileKey,
        pixels: Vec<[u8; 4]>,
    },
    SetOpacity {
        object_id: ObjectId,
        value: f32,
    },
    Undo,
    Redo,
}

impl AppState {
    pub fn apply(&mut self, cmd: UiCmd) {
        match cmd {
            UiCmd::AddLayer { name } => {
                let id = self.layers.insert(Layer {
                    name,
                    visible: true,
                    locked: false,
                });
                self.layer_order.push(id);
                self.layer_objects.insert(id, Vec::new());
            }
            UiCmd::RemoveLayer { id } => {
                self.layers.remove(id);
                self.layer_order.retain(|x| *x != id);
                if let Some(objs) = self.layer_objects.remove(&id) {
                    for obj in objs {
                        self.objects.remove(obj);
                    }
                }
            }
            UiCmd::MoveLayer { id, new_index } => {
                if let Some(cur) = self.layer_order.iter().position(|x| *x == id) {
                    let item = self.layer_order.remove(cur);
                    let idx = new_index.min(self.layer_order.len());
                    self.layer_order.insert(idx, item);
                }
            }
            UiCmd::AddImageObject { layer } => {
                if self.layers.contains_key(layer) {
                    let obj = self.objects.insert(Object::Image {
                        tiles: HashMap::new(),
                        blend: BlendMode::Normal,
                        opacity: 1.0,
                    });
                    self.layer_objects.entry(layer).or_default().push(obj);
                }
            }
            UiCmd::PaintTile {
                object_id,
                key,
                pixels,
            } => {
                if let Some(Object::Image { tiles, .. }) = self.objects.get_mut(object_id) {
                    tiles.insert(key, Tile { pixels });
                }
            }
            UiCmd::SetOpacity { object_id, value } => {
                if let Some(Object::Image { opacity, .. }) = self.objects.get_mut(object_id) {
                    *opacity = value.clamp(0.0, 1.0);
                }
            }
            UiCmd::Undo | UiCmd::Redo => {}
        }
    }
}

pub fn tile_coords(x: i32, y: i32) -> (TileKey, (u32, u32)) {
    let tx = x.div_euclid(TILE_SIZE as i32);
    let ty = y.div_euclid(TILE_SIZE as i32);
    let lx = x.rem_euclid(TILE_SIZE as i32) as u32;
    let ly = y.rem_euclid(TILE_SIZE as i32) as u32;
    ((tx, ty), (lx, ly))
}
