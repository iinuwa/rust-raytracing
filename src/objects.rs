use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};
use rand::prelude::*;

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

pub trait Material<T> {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub scattered_direction: Ray,
    pub attenuation: Vec3,
}

trait Reflect {
    fn reflect(vector: &Vec3, unit_vector: &Vec3) -> Vec3 {
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

impl<T> Material<T> for Lambertian {
    #[allow(unused_variables)]
    fn scatter(&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult> {
        let target = &hit_record.point + &hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point.clone(), target - hit_record.point);
        Some(ScatterResult {
            scattered_direction: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f32) -> Self {
        let f = if fuzziness < 1.0 { fuzziness } else { 1.0 };
        Metal {
            albedo,
            fuzziness: f,
        }
    }
}

impl Reflect for Metal {}

impl<T> Material<T> for Metal {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult> {
        let reflected = Self::reflect(&Vec3::unit_vector(ray.direction()), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzziness * random_in_unit_sphere(),
        );
        if Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0 {
            return Some(ScatterResult {
                scattered_direction: scattered,
                attenuation: self.albedo.clone(),
            });
        }
        None
    }
}

trait Refract {
    fn refract(vector: &Vec3, vector2: &Vec3, refractive_index: f32) -> Option<Vec3> {
        let unit_vector = vector.unit_vector();
        let dt = Vec3::dot(&unit_vector, vector2);
        let discriminant = 1.0 - refractive_index.powi(2) * (1.0 - dt.powi(2));
        if discriminant > 0.0 {
            let refracted =
                refractive_index * (unit_vector - dt * vector2) - discriminant.sqrt() * vector2;
            return Some(refracted);
        }
        None
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl Refract for Dielectric {}
impl Reflect for Dielectric {}
impl<T> Material<T> for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult> {
        let outward_normal: Vec3;
        let reflected = Self::reflect(ray.direction(), &hit_record.normal);
        let refractive_index: f32;
        if Vec3::dot(ray.direction(), &hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            refractive_index = self.refractive_index;
        } else {
            outward_normal = hit_record.normal;
            refractive_index = 1.0 / self.refractive_index;
        }

        let attenuation = Vec3(1.0, 1.0, 0.0);
        if let Some(refracted) = Self::refract(ray.direction(), &outward_normal, refractive_index) {
            return Some(ScatterResult {
                scattered_direction: Ray::new(hit_record.point, refracted),
                attenuation,
            });
        } else {
            return Some(ScatterResult {
                scattered_direction: Ray::new(hit_record.point, reflected),
                attenuation,
            });
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut point: Vec3;
    while {
        point = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3(1.0, 1.0, 1.0);
        point.squared_length() >= 1.0
    } {}
    point
}
