use nalgebra_glm::Vec3;

pub enum Light {
    Ambient { intensity: f32 },
    Point { intensity: f32, position: Vec3 },
    Directional { intensity: f32, direction: Vec3 },
}

impl Light {
    pub fn intensity(&self) -> f32 {
        match self {
            Light::Ambient { intensity } => *intensity,
            Light::Point { intensity, .. } => *intensity,
            Light::Directional { intensity, .. } => *intensity,
        }
    }
}