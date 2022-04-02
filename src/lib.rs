#![allow(unused_parens)]

use gms_binder::*;
use config_box::file_watcher::FileWatcherConfigBox;
use crossbeam_epoch::{self as epoch};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::time::Instant;

static mut GLOBAL_ROPEWORLD: Option<rope_lib::GlobalState> = None;
static mut CONFIG_BOX : Option<FileWatcherConfigBox> = None;

gms_bind_start!("ld50_lib", "ld50_lib.dll", "lib");

#[no_mangle]
#[gms_bind]
pub extern "C" fn dummy() -> f64 {
    println!("Hello");
    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn setup_config_box(path_raw : *const c_char) -> f64 {
    unsafe {
        let path = CStr::from_ptr(path_raw).to_str().unwrap();
        CONFIG_BOX = Some(FileWatcherConfigBox::new(path).expect("Could not create configbox"));
    }

    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn config_box_tick() -> f64 {
    unsafe {
        let config_box = CONFIG_BOX.as_ref().unwrap();
        config_box.refresh();
    }

    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn config_f64(path_raw : *const c_char) -> f64 {
    unsafe {
        let config_box = CONFIG_BOX.as_ref().unwrap();
        let path = CStr::from_ptr(path_raw).to_str().unwrap();
        let guard = &epoch::pin();
        let root = config_box.root(guard);
        root.get_f32(path) as f64
    }
}

#[no_mangle]
#[gms_bind]
// Warning leaks
pub extern "C" fn config_str(path_raw : *const c_char) -> *const c_char  {
    unsafe {
        let config_box = CONFIG_BOX.as_ref().unwrap();
        let path = CStr::from_ptr(path_raw).to_str().unwrap();
        let guard = &epoch::pin();
        let root = config_box.root(guard);

        let value_str = root.get_str(path);

        let cstring = CString::new(value_str).unwrap();
        let cstring_ptr = cstring.as_ptr();
        std::mem::forget(cstring);
        cstring_ptr
    }
}


#[no_mangle]
#[gms_bind]
pub extern "C" fn rope_reset() -> f64 {
    unsafe {
        GLOBAL_ROPEWORLD = Some(rope_lib::GlobalState::new());
    }
    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_node(x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state.world.add_node(x as f32, y as f32);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_box(x: f64, y: f64, size : f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state.world.add_box_collider(x as f32, y as f32, size as f32);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn set_fixed(nid: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let mut node = state.world.get_node_mut(nid.round() as usize);
        node.node_type = rope_lib::NodeType::Fixed;
        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn set_node_pos(nid: f64, x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let pos = rope_lib::Vec2::new(x as f32, y as f32);
        state.world.set_node_pos_respect_colliders(nid.round() as usize, pos);
        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_rope(from: f64, to: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state
            .world
            .add_rope(from.round() as usize, to.round() as usize);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_rope_length(from: f64, to: f64, len : f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let id = state
            .world
            .add_rope_length(from.round() as usize, to.round() as usize, len as f32);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn tick() -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.t += 1;

        let new_last_tick = Instant::now();
        let since_start = new_last_tick.duration_since(state.last_tick);
        let micros_since = since_start.as_micros() as f32;
        const SIXTY_FPS_DUR_MICROS: f32 = 1_000_000.0 / 60.0;
        let norm_dt = micros_since / SIXTY_FPS_DUR_MICROS;

        state.world.tick(norm_dt);
        state.last_tick = new_last_tick;

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn dry_tick() -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.last_tick = Instant::now();

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_x(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.get_node(id.round() as usize).pos.x as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_y(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.get_node(id.round() as usize).pos.y as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn toggle_node(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        let mut node = state.world.get_node_mut(id.round() as usize);
        node.node_type = match node.node_type {
            rope_lib::NodeType::Free => rope_lib::NodeType::Fixed,
            rope_lib::NodeType::Fixed => rope_lib::NodeType::Free,
        };

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_type(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_ref().unwrap();
        match state.world.get_node(id.round() as usize).node_type {
            rope_lib::NodeType::Free => 0.0,
            rope_lib::NodeType::Fixed => 1.0,
        }
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_rope_broken(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_ref().unwrap();
        if (state.world.get_rope(id.round() as usize).broken) {
            1.0
        } else {
            0.0
        }
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_rope_from(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_ref().unwrap();
        state.world.get_rope(id.round() as usize).from as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_rope_to(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_ref().unwrap();
        state.world.get_rope(id.round() as usize).to as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_sim_t() -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.t as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_static_force(x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.forces.push(Box::new(rope_lib::ConstantForce {
            force: rope_lib::Vec2::new(x as f32, y as f32),
        }));

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_inverse_square_force(strength: f64, x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_ROPEWORLD.as_mut().unwrap();
        state.world.forces.push(Box::new(rope_lib::InverseSquareForce {
            strength: strength as f32,
            pos: rope_lib::Vec2::new(x as f32, y as f32),
        }));

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn free_string(s: *mut c_char) -> f64 {
    unsafe {
        if (!s.is_null()) {
            let _ = CString::from_raw(s);
        }

        0.0
    }
}

gms_bind_end!();