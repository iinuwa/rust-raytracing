use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};

pub struct HitRecord<'a, T> {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material<T>,
}

pub trait Hittable<T> {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord<T>>;
}

pub struct Sphere<'a, T> {
    pub center: Vec3,
    pub radius: f32,
    pub material: &'a dyn Material<T>,
}

impl<'a, T> Hittable<T> for Sphere<'a, T> {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord<T>> {
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
                    material: self.material,
                };
                return Some(hit_record);
            }
        }
        None
    }
}

pub struct HittableList<'a, T> {
    list: Vec<&'a dyn Hittable<T>>,
}

impl<'a, T> HittableList<'a, T> {
    pub fn new(list: Vec<&'a dyn Hittable<T>>) -> Self {
        HittableList { list }
    }
}

impl<T> Hittable<T> for HittableList<'_, T> {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord<T>> {
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

pub struct ScatterResult {
    pub scattered_direction: Ray,
    pub attenuation: Vec3,
}
