use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {orig: orig, dir: dir}
    }

    pub fn origin(self) -> Point3 {return self.orig;}
    pub fn direction(self) -> Vec3 {return self.dir;}

    pub fn at(self, t: f32) -> Point3 {
        return self.orig + t * self.dir;
    }
}
