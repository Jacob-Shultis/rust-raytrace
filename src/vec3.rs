use std::ops;
use std::fmt;

#[derive(Copy, Clone)]
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

    pub fn length(&self) -> f32 {
        return f32::sqrt(self.length_squared());
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self { e: [
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]
        ] }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.clone() / self.length()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Self { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Self { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Self { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Self { e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]] }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self] }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Self { e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs] }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Self { e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]] }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [rhs.e[0] / self, rhs.e[1] / self, rhs.e[2] / self] }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Self { e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs] }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.e[0], self.e[1], self.e[2])
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {e: [0.0, 0.0, 0.0]}
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
