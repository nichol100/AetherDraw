// Physics module scaffold: rigid body, soft body, cloth, particles, forces.

pub struct PhysicsWorld {
    pub gravity: glam::Vec3,
    pub wind: glam::Vec3,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            gravity: glam::Vec3::new(0.0, -9.81, 0.0),
            wind: glam::Vec3::ZERO,
        }
    }
}

pub enum BodyType {
    Rigid,
    Soft,
    Cloth,
    Particle,
}