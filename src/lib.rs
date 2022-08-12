#![no_std]
// #[cfg(feature = "buddy-alloc")]

mod wasm4;
mod map;
mod utils;
mod ecs;
use utils::*;
use ecs::*;

#[no_mangle]
unsafe fn start() {
    let mut world = World::new();
    GLOBAL_WORLD = Some(world);
}

static mut GLOBAL_WORLD: Option<World> = None;
static mut PREVIOUS_MOUSE: u8 = 0;
static mut FRAME_NUMBER: u32 = 0;

#[no_mangle]
unsafe fn update() {
    FRAME_NUMBER += 1;
    if let Some(world) = &mut GLOBAL_WORLD {
        world.render_map();
    }
}
