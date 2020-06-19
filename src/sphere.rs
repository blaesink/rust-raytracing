use crate::vec3::{Vec3,Point3,dot};
use crate::ray::Ray;
use crate::hittable::{HitRecord,Hittable};

#[derive(Default,Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Self {
        Self {
            center: cen,
            radius: r
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = dot(&oc, &r.direction());
        let c: f64 = &oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();
            let mut temp: f64 = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r,&outward_normal);
                return true;
            } else {
                temp = (-half_b + root) / a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.at(rec.t);
                    let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                    rec.set_face_normal(&r,&outward_normal);

                    return true;
                }
            }
        }

        false
    }
}
