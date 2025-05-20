use std::{ops::DerefMut, slice::from_raw_parts_mut, sync::Mutex};

use pixels::PixelBuffer;
use world::World;

pub mod camera;
pub mod geometry;
mod operations;
mod painter;
pub mod pixels;
pub mod scene;
pub mod shaders;
pub mod world;

// Global Lock
static WORLD: Mutex<Option<World>> = Mutex::new(None);

fn with_world_opt_mut(f: impl FnOnce(&mut Option<World>)) -> bool {
    let guard = WORLD.lock();
    if guard.is_err() {
        return false;
    }
    let mut opt = guard.unwrap();
    f(opt.deref_mut());
    true
}

fn with_world(f: impl FnOnce(&World)) -> bool {
    let guard = WORLD.lock();
    if guard.is_err() {
        return false;
    }
    let opt = guard.unwrap();
    if opt.is_none() {
        return false;
    }
    f(opt.as_ref().unwrap());
    true
}

fn with_world_mut(f: impl FnOnce(&mut World)) -> bool {
    let guard = WORLD.lock();
    if guard.is_err() {
        return false;
    }
    let mut opt = guard.unwrap();
    if opt.is_none() {
        return false;
    }
    f(opt.as_mut().unwrap());
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn rusterizer_init_scene(height: u32, width: u32) -> bool {
    with_world_opt_mut(|world_opt| {
        world_opt.replace(World::new(height, width));
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn rusterizer_deinit_world() -> bool {
    with_world_opt_mut(|world_opt| {
        let prev_world = world_opt.take();
        drop(prev_world);
    })
}

/// # Safety
///
/// It's caller's responsibility to provide a pixel buffer that
/// can contain pixel_num * 4 bytes data.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rusterizer_draw_to_pixel_buf(buf: *mut u8) -> bool {
    with_world(|world| {
        // TODO: assuming ARGB here (32bits), can make it configurable.
        //let buf_len = world.get_canvas_size() as usize * 4;
        let (height, width) = world.get_canvas_size();
        let buf_len = (height * width) as usize * 4;
        let buf = unsafe { from_raw_parts_mut(buf, buf_len) };
        let pixel_buf = PixelBuffer::new(height, buf);
        world.draw(pixel_buf);
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn rusterizer_camera_yaw(yaw: f32) {
    with_world_mut(|world| world.set_yaw(yaw));
}

#[unsafe(no_mangle)]
pub extern "C" fn rusterizer_camera_ztranslate(amount: f32) {
    with_world_mut(|world| world.set_z_translate(amount));
}

#[unsafe(no_mangle)]
pub extern "C" fn rusterizer_light_position_offset(x: f32, y: f32, z: f32) {
    with_world_mut(|world| world.update_light_offset(x, y, z));
}
