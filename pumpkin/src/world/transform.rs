use pumpkin_core::math::vector3::Vector3;

pub struct Transform {
    pub position: Vector3<f64>,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl Transform {
    pub fn new(position: Vector3<f64>, yaw: f32, pitch: f32, on_ground: bool) -> Self {
        Self {
            position,
            yaw,
            pitch,
            on_ground,
        }
    }
}
