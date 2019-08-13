use crate::vec3::Vec3;

pub struct Ray {
    point: Vec3,
    vector: Vec3,
}

impl Ray {
    pub fn new(point: Vec3, vector: Vec3) -> Self {
        Ray { point, vector }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.point
    }
    pub fn direction(&self) -> &Vec3 {
        &self.vector
    }
    pub fn point_at(&self, t: f32) -> Vec3 {
        let vector = t * &self.vector;
        &self.point + &vector
    }
}
