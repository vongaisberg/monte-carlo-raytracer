use rand::{thread_rng, Rng};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

// Yay const generics!

type Vec3 = [f64; 3];

#[derive(Debug, Clone, Copy)]
pub struct Vector(Vec3);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Self([x, y, z])
    }
    pub fn random_on_unit_sphere() -> Vector {
        let mut rng = thread_rng();
        Self::new(
            (rng.gen::<f64>() * 2f64) - 1f64,
            (rng.gen::<f64>() * 2f64) - 1f64,
            (rng.gen::<f64>() * 2f64) - 1f64,
        )
        .normalize()
    }
    pub fn random_in_unit_sphere() -> Vector {
        let mut rng = thread_rng();
        let vec = Self::new(
            (rng.gen::<f64>() * 2f64) - 1f64,
            (rng.gen::<f64>() * 2f64) - 1f64,
            (rng.gen::<f64>() * 2f64) - 1f64,
        );
        if vec.length() >= 1f64 {
            vec
        } else {
            Self::random_in_unit_sphere()
        }
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

    pub fn reflect(self, normalized_other: &Self) -> Self {
        self - *normalized_other * (self.dot(normalized_other) * 2f64)
    }

    pub fn refract(self, normalized_other: &Self, refraction_fraction: f64) -> Option<Self> {
        let dt = self.dot(&normalized_other);
        let discriminant = 1f64 - refraction_fraction * refraction_fraction * (1f64 - dt * dt);
        if discriminant > 0f64 {
            Some(
                (self - *normalized_other * dt) * refraction_fraction
                    - *normalized_other * ((discriminant).sqrt()),
            )
        } else {
            None
        }
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

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Self([r / 255f64, g / 255f64, b / 255f64])
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
    pub fn pow(self, rhs: f64) -> Self {
        Self([
            self.0[0].powf(rhs),
            self.0[1].powf(rhs),
            self.0[2].powf(rhs),
        ])
    }
    pub fn abs(self) -> Self {
        Self([self.0[0].abs(), self.0[1].abs(), self.0[2].abs()])
    }
    pub fn less_than(&self, rhs: Color) -> bool {
        self.r() < rhs.r() && self.g() < rhs.g() && self.b() < rhs.b()
    }
    pub const BLACK: Self = Self([0f64, 0f64, 0f64]);
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

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}
