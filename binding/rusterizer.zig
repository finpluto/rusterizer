pub extern fn rusterizer_init_scene(h: u32, w: u32) bool;
pub extern fn rusterizer_deinit_world() bool;
pub extern fn rusterizer_draw_to_pixel_buf(buf: [*c]u8) bool;
pub extern fn rusterizer_camera_yaw(yaw: f32) void;
pub extern fn rusterizer_light_position_offset(x: f32, y: f32, z: f32) void;
