use crate::objects::*;
use crate::ray::Ray;
use crate::vec3::{Vec3, Vector};
use rand::prelude::*;

pub trait Material<T> {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult>;
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
        let discriminant = 1.0 - refractive_index * refractive_index * (1.0 - dt * dt);
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

    fn schlick(cosine: f32, refractive_index: f32) -> f32 {
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Refract for Dielectric {}
impl Reflect for Dielectric {}
impl<T> Material<T> for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord<T>) -> Option<ScatterResult> {
        let outward_normal: Vec3;
        let refractive_index: f32;
        let mut cosine: f32;
        if Vec3::dot(ray.direction(), &hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal.clone();
            refractive_index = self.refractive_index;
            // cosine = self.refractive_index * -Vec3::dot(ray.direction(), &hit_record.normal)
            //     / ray.direction().length();
            cosine = Vec3::dot(ray.direction(), &hit_record.normal) / ray.direction().length();
            cosine = 1.0 - self.refractive_index * self.refractive_index * (1.0 - cosine * cosine);
        } else {
            outward_normal = hit_record.normal.clone();
            refractive_index = 1.0 / self.refractive_index;
            cosine = -Vec3::dot(ray.direction(), &hit_record.normal) / ray.direction().length();
        }

        let mut reflect_probability = 1.0;
        let refracted = Self::refract(ray.direction(), &outward_normal, refractive_index);
        if refracted.is_some() {
            reflect_probability = Self::schlick(cosine, refractive_index);
        }

        let attenuation = Vec3(1.0, 1.0, 1.0);
        let mut rng = rand::thread_rng();
        let random_num: f32 = rng.gen();
        if random_num < reflect_probability {
            let reflected = Self::reflect(ray.direction(), &hit_record.normal);
            return Some(ScatterResult {
                scattered_direction: Ray::new(hit_record.point, reflected),
                attenuation,
            });
        } else {
            return Some(ScatterResult {
                scattered_direction: Ray::new(hit_record.point, refracted.unwrap()),
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
