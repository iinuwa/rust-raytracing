use crate::vec3::Vector3;

struct Ray {
    point: &Vector3,
    vector: &Vector3,
}

impl Ray {
    pub fn origin(&self) -> Vector3 {
        return self.point;
    }
    pub fn direction(&self) -> Vector3 {
        return self.vector;
    }
    pub fn point_at(&self, t: f32) -> Vector3 {
        return self.point + t * self.vector;
    }
}
