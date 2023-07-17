use crate::vec3::{Point3, Vec3};
use crate::hittable::Hittable;

pub struct Sphere {
    cen: Point3;
    r: f32;
}

impl Sphere {
    pub fn new(cen: Point3, r: f32) -> Self {
        Self { cen: cen, r: r }
    }
}

impl Hittable for Sphere {
    fn hit(r: Ray, t_min: f32, t_max: f32, rec: HitRecord) -> bool {
        let oc: Vec3 = r.origin() - center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = oc.dot(r.direction());
        let c: f32 = oc.length_squared() - radius*radius;
        let discriminant: f32 = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f32 = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let root: f32 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - center) / radius;

        return true;
    }
}
