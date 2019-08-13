use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};

pub struct HitRecord {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord> {
        let origin_offset = ray.origin() - &self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(&origin_offset, ray.direction());
        let c = Vec3::dot(&origin_offset, &origin_offset) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let hit_record = HitRecord {
                    distance,
                    point: ray.point_at(distance),
                    normal: (ray.point_at(distance) - &self.center) / self.radius,
                };
                return Some(hit_record);
            }
        }
        None
    }
}

pub struct HittableList<'a> {
    list: Vec<&'a Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new(list: Vec<&'a Hittable>) -> Self {
        HittableList { list }
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = distance_max;
        for object in self.list.iter() {
            if let Some(i) = object.hit(ray, distance_min, closest_so_far) {
                closest_so_far = i.distance;
                result = Some(i)
            }
        }
        result
    }
}
