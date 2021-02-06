#[derive(Debug, Clone)]
pub struct Viewport {
    width: i32,
    height: i32,
    device_scale_factor: i32,
    is_mobile: bool,
    is_landscape: bool,
    has_touch: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            width: 800,
            height: 800,
            device_scale_factor: 1,
            is_mobile: false,
            is_landscape: false,
            has_touch: false,
        }
    }
}
