use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};
use rand::prelude::*;

pub struct HitRecord<'a, T>
where
    T: Material {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a T
}

pub trait Hittable<T> where T: Material {
    fn hit(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<HitRecord<T>>;
}

pub struct Sphere<T> 
where
    T: Material,
     {
    pub center: Vec3,
    pub radius: f32,
    pub material: T,
}

impl <T> Hittable<T> for Sphere<T> where T: Material, {
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
                    material: &self.material,
                };
                return Some(hit_record);
            }
        }
        None
    }
}

pub struct HittableList<'a, T> {
    list: Vec<&'a Hittable<T>>,
}

impl<'a, T> HittableList<'a, T> {
    pub fn new(list: Vec<&'a Hittable<T>>) -> Self {
        HittableList { list }
    }
}

impl <T> Hittable<T> for HittableList<'_, T> where T: Material {
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

pub trait Material {
    fn scatter<T> (&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult>
    where T: Material;
}

pub struct ScatterResult {
    pub scattered_direction: Ray,
    pub attenuation: Vec3,
}

trait Reflect {
    fn reflect (vector: &Vec3, unit_vector: &Vec3) -> Vec3 {
       vector - &(2.0 * Vec3::dot(vector, unit_vector) * unit_vector)
    }
}

pub struct Lambertian {
    /*
     * Albedo is the measure of the diffuse reflection of solar radiation out of the
     * total solar radiation received by an astronomical body. It is dimensionless
     * and measured on a scale from 0 to 1.
     */
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused_variables)]
    fn scatter<T: Material> (&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult> {
        let target = &hit_record.point + &hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(
            hit_record.point.clone(),
            target - hit_record.point,
        );
        Some(ScatterResult {
            scattered_direction: scattered,
            attenuation: self.albedo.clone()
        })
    }
}

struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Reflect for Metal {}

impl Material for Metal {
    fn scatter<T: Material> (&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult> {
        let reflected = Self::reflect(&Vec3::unit_vector(ray.direction()), &hit_record.normal);
        let scattered = Ray::new( hit_record.point, reflected,);
        if Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0 {
            return Some(ScatterResult{ scattered_direction: scattered, attenuation: self.albedo.clone() });
        }
        None
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut point: Vec3;
    while { 
        point = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3(1.0, 1.0, 1.0);
        point.squared_length() >= 1.0
    }{}
    point
}