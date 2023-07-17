use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3;
    normal: Vec3;
    t: f32;
    front_face: bool;
}

impl HitRecord {
    fn set_face_normal(self, r: Ray, outword_normal: Vec3) {
        self.front_face = r.direction().dot(outword_normal);
        self.normal = self.front_face ? outword_normal : -outword_normal;
    }
}

pub trait Hittable {
    fn hit(r: Ray, t_min: f32, t_max: f32, rec: HitRecord) -> bool;
}