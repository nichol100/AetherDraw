// 3D viewport scaffold (wgpu-based). Meshes, camera, lights.

pub struct Camera3D {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub fov_y: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            eye: glam::Vec3::new(0.0, 2.0, 5.0),
            target: glam::Vec3::ZERO,
            up: glam::Vec3::Y,
            fov_y: 60.0_f32.to_radians(),
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<glam::Vec3>,
    pub indices: Vec<u32>,
}