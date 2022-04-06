#![allow(unused_parens)]

mod wasm_instant;

use ld50_lib::*;
use wasm_bindgen::prelude::*;
use wasm_instant::WasmInstant;

pub struct GlobalState {
    pub t: usize,
    pub world: World,
    pub last_tick: WasmInstant,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            t: 0,
            world: World::default(),
            last_tick: WasmInstant::now(),
        }
    }
}
static mut GLOBAL_ROPEWORLD: Option<GlobalState> = None;

#[wasm_bindgen]
pub fn reset() -> f64 {
    unsafe {
        GLOBAL_ROPEWORLD = Some(GlobalState::new());
        0.0
    }
}

#[wasm_bindgen]
pub fn add_node(x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state.world.add_node(x as f32, y as f32);
        id as f64
    }
}

#[wasm_bindgen]
pub fn add_box(x: f64, y: f64, size : f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state.world.add_box_collider(x as f32, y as f32, size as f32);
        id as f64
    }
}

#[wasm_bindgen]
pub fn set_fixed(nid: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let mut node = state.world.get_node_mut(nid.round() as usize);
        node.node_type = NodeType::Fixed;
        0.0
    }
}

#[wasm_bindgen]
pub fn set_free(nid: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let mut node = state.world.get_node_mut(nid.round() as usize);
        node.node_type = NodeType::Free;
        0.0
    }
}

#[wasm_bindgen]
pub fn set_node_pos(nid: f64, x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let pos = Vec2::new(x as f32, y as f32);
        let mut node = state.world.get_node_mut(nid.round() as usize);
        node.pos = pos;
        0.0
    }
}

#[wasm_bindgen]
pub fn set_node_pos_player(nid: f64, x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let pos = Vec2::new(x as f32, y as f32);
        state.world.set_node_pos_respect_colliders(nid.round() as usize, pos);
        0.0
    }
}

#[wasm_bindgen]
pub fn get_tension() -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.get_tension() as f64
    }
}

#[wasm_bindgen]
pub fn add_rope(from: f64, to: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state
            .world
            .add_rope(from.round() as usize, to.round() as usize);
        id as f64
    }
}

#[wasm_bindgen]
pub extern "C" fn add_rope_length(from: f64, to: f64, len : f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state
            .world
            .add_rope_length(from.round() as usize, to.round() as usize, len as f32);
        id as f64
    }
}

#[wasm_bindgen]
pub extern "C" fn tick() -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.t += 1;

        let new_last_tick = WasmInstant::now();
        let since_start = new_last_tick.duration_since(state.last_tick);
        let micros_since = since_start.as_micros() as f32;
        const SIXTY_FPS_DUR_MICROS: f32 = 1_000_000.0 / 60.0;
        let norm_dt = micros_since / SIXTY_FPS_DUR_MICROS;

        state.world.tick(norm_dt);
        state.last_tick = new_last_tick;

        0.0
    }
}

#[wasm_bindgen]
pub fn get_node_x(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.get_node(id.round() as usize).pos.x as f64
    }
}

#[wasm_bindgen]
pub fn get_node_y(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.get_node(id.round() as usize).pos.y as f64
    }
}