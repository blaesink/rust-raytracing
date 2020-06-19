use crate::vec3::{Vec3,Point3,dot};
use crate::ray::Ray;
//use std::rc::Rc;

#[derive(Default,Clone,Copy)]
pub struct HitRecord {
    pub front_face: bool,
    pub normal: Vec3,
    pub p: Point3,
    pub t: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0 as f64;

        self.normal = match self.front_face {
            true => *outward_normal,
            false => -1.0 * *outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        false
    }
}

#[derive(Default,Debug)]
pub struct HittableList<T> {
    objects: Vec::<T>
}

impl<T> HittableList<T> {
    pub fn new(o: Vec::<T>) -> Self {
        Self {
            objects: o
        }
    }
    pub fn add(&mut self, o: T) {
        self.objects.push(o);
    }
}

impl<T> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for obj in &self.objects {
            if self.hit(&r,t_min,closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
