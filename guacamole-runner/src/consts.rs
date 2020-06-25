pub mod textures {
    pub const FLOOR: &'static str = "hex-grass";
    pub const FLOOR_BRICK: &'static str = "hex-stone-floor";
    pub const WALL: &'static str = "hex-dirt";
    pub const WALL_BRICK: &'static str = "hex-stone";
    pub const MARKER: &'static str = "marker";
}

pub mod draw_layers {
    pub const FLOOR: f32 = 0.0;
    pub const WALL: f32 = 1.0;
}

pub const FLOOR_WIDTH: f32 = 36.0;
pub const FLOOR_HEIGHT: f32 = 36.0;
pub const FLOOR_VERT_STEP: f32 = 28.0;
pub const FLOOR_DEPTH_STEP: f32 = 12.0;

pub const WALL_VERT_OFFSET: f32 = 12.0;
pub const WALL_VERT_STEP: f32 = 12.0;

pub const CAM_SPEED: f32 = 5.0;

pub const MAX_FLOOR_HEIGHT: u8 = 2;
pub const MAX_BRICK_HEIGHT: u8 = 4;

pub const WIDTH: usize = 10000;
pub const HEIGHT: usize = 10;

pub const CLEAR_COL: crate::tetra::graphics::Color = crate::tetra::graphics::Color::rgb(0.392, 0.584, 0.929);

pub const SCROLL_RATE: f32 = 4.;