use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};
use rand::prelude::*;
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    /// v_fov is the vertical field of view in degress, from top to bottom.
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        v_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = v_fov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);
        //let lower_left_corner = Vec3(-half_width, -half_height, -1.0);
        let lower_left_corner = &look_from
            - &(half_width * focus_distance * &u)
            - half_height * focus_distance * &v
            - focus_distance * &w;
        let horizontal = 2.0 * half_width * focus_distance * &u;
        let vertical = 2.0 * half_height * focus_distance * &v;

        let lens_radius = aperture / 2.0;
        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let horizontal_vector = u * &self.horizontal;
        let vertical_vector = v * &self.vertical;
        let ray_direction = self.lens_radius * random_in_unit_disk();
        let offset = ray_direction.x() * &self.u + ray_direction.y() * &self.v;
        let direction =
            &self.lower_left_corner + &horizontal_vector + vertical_vector - &self.origin - &offset;
        Ray::new(self.origin.clone() + offset, direction)
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut point: Vec3;
    let mut rng = rand::thread_rng();
    while {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        point = 2.0 * Vec3(x, y, 0.0) - Vec3(1.0, 1.0, 0.0);
        Vec3::dot(&point, &point) >= 1.0
    } {}
    point
}
