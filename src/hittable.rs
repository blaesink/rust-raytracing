use crate::vec3::{Vec3,Point3};
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32
}

pub trait Hittable {
    fn hit<T>(r: &Ray, obj: T, t_min: f32, t_max: f32, rec: &HitRecord) -> bool {
        false
    }
}
