use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

// Yay const generics!
type Vec3 = [f64; 3];

pub struct Vector(Vec3);
pub struct Color(Vec3);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Self([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Self([r, g, b])
    }

    pub fn r(&self) -> f64 {
        self.0[0]
    }
    pub fn g(&self) -> f64 {
        self.0[1]
    }
    pub fn b(&self) -> f64 {
        self.0[2]
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Vector {
    pub fn length(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn normalize(&self) -> Self {
        Self([
            self.x() / self.length(),
            self.y() / self.length(),
            self.z() / self.length(),
        ])
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ])
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}
