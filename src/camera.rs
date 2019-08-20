use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// v_fov is the vertical field of view in degress, from top to bottom.
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, v_fov: f32, aspect: f32) -> Self {
        let theta = v_fov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);
        //let lower_left_corner = Vec3(-half_width, -half_height, -1.0);
        let lower_left_corner = &look_from - &(half_width * &u) - half_height * &v - w;
        let horizontal = 2.0 * half_width * &u;
        let vertical = 2.0 * half_height * &v;

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let horizontal_vector = u * &self.horizontal;
        let vertical_vector = v * &self.vertical;
        let direction =
            &self.lower_left_corner + &horizontal_vector + vertical_vector - &self.origin;
        Ray::new(self.origin.clone(), direction)
    }
}
