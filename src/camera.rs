use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Vec3(0.0, 0.0, 0.0);
        let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
        let horizontal = Vec3(4.0, 0.0, 0.0);
        let vertical = Vec3(0.0, 2.0, 0.0);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let horizontal_vector = u * &self.horizontal;
        let vertical_vector = v * &self.vertical;
        let direction = &self.lower_left_corner + &horizontal_vector + vertical_vector;
        Ray::new(self.origin.clone(), direction)
    }
}
