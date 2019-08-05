use crate::vec3::Vector3;

struct Ray {
    a: &Vector3,
    b: &Vector3,
}

impl Ray {
    pub fn origin(&self) -> Vector3 {
        return self.a;
    }
    pub fn direction(&self) -> Vector3 {
        return self.b;
    }
    pub fn point_at(&self, t: f32) -> Vector3 {
        return self.a + t * self.b;
    }
}
