use std::ops;

pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self {e: [e0, e1, e2]}
    }

    pub fn x(self) -> f32 {return self.e[0];}
    pub fn y(self) -> f32 {return self.e[1];}
    pub fn z(self) -> f32 {return self.e[2];}

    pub fn length(self) -> f32 {
        return f32::sqrt(self.length_squared());
    }

    pub fn length_squared(self) -> f32 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Self { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Self { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Self { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {e: [0.0, 0.0, 0.0]}
    }
}

type Point3 = Vec3;
type Color = Vec3;
