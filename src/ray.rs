use crate::vec3::{Vec3};

pub struct Ray<'a, T> {
    point: &'a Vec3<T>,
    vector: &'a Vec3<T>,
}

/*
impl <T: std::marker::Sized> Ray<T> {
    pub fn origin(self) -> Vec3<T> {
        return self.point;
    }
    pub fn direction(self) -> Vec3<T> {
        return self.vector;
    }
    pub fn point_at(self, t: T) -> Vec3<T> {
        return self.point + self.vector * t;
    }
}
*/

impl <'a> Ray<'a, f32> {
    pub fn origin(&self) -> &'a Vec3<f32> {
        self.point
    }
    pub fn direction(&self) -> &'a Vec3<f32> {
        &self.vector
    }
    pub fn point_at(&self, t: f32) -> &'a Vec3<f32> {
        self.point + self.vector * t
    }
}
