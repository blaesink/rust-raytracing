use crate::vec3::{Vec3,Point3};
use crate::ray::Ray;
use crate::hittable::{HitRecord,Hittable};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(cen: Point3, r: f32) -> Self {
        Self {
            center: cen,
            radius: r
        }
    }
}

impl Hittable for Sphere {
    // override the default trait method here
    fn hit<Sphere>(r: &Ray, obj: Sphere, t_min: f32, t_max: f32, rec: &HitRecord) -> bool {
        // original C++ code used `center` as an argument, but because of how traits work, I adapted the original Trait to accept an `obj` argument with a generic type for future use
        let oc: Vec3 = obj.center;

        false
    }
}
