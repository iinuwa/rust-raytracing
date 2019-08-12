use crate::vec3::{Vec3};

pub  struct Ray<'a> {
    point: &'a Vec3,
    vector: &'a Vec3,
}

impl <'a> Ray<'a> {
    pub fn new(point: &'a Vec3, vector: &'a Vec3) -> Self {
        Ray {
            point,
            vector,
        }
    }
    pub fn origin(&self) -> &Vec3 {
        self.point
    }
    pub fn direction(&self) -> &Vec3 {
        self.vector
    }
    pub fn point_at(&self, t: f32) -> Vec3 {
        let vector = t * self.vector;
        self.point + &vector
    }
}
