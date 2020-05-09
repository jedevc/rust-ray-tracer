use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformFloat, UniformSampler};
use rand::prelude::*;
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Point = Vec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Self {
        self.clone() / self.length()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * self.dot(normal) * 2.0
    }

    pub fn refract(self, normal: Self, etai_etat: f64) -> Self {
        let cos_theta = normal.dot(-self);
        let r_parallel = (self + normal * cos_theta) * etai_etat;
        let r_perp = normal * -(1.0 - r_parallel.length_squared()).sqrt();
        r_parallel + r_perp
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub fn random_sphere_point<R: Sized + Rng>(rng: &mut R) -> Point {
    let min = Vec::new(0.0, 0.0, 0.0);
    let max = Vec::new(1.0, 1.0, 1.0);
    loop {
        let p = rng.gen_range(min, max);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_hemisphere_point<R: Sized + Rng>(rng: &mut R, normal: Vec) -> Point {
    let p = random_sphere_point(rng);
    if p.dot(normal) > 0.0 {
        p
    } else {
        -p
    }
}

pub fn random_lambertian_point<R: Sized + Rng>(rng: &mut R) -> Point {
    let a: f64 = rng.gen_range(0.0, 2.0 * PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();

    Point {
        x: r * a.cos(),
        y: r * a.sin(),
        z,
    }
}

impl Add for Vec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vec {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Mul<f64> for Vec {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign<f64> for Vec {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div<f64> for Vec {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        self * (1.0 / scalar)
    }
}

impl DivAssign<f64> for Vec {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UniformVec {
    x: UniformFloat<f64>,
    y: UniformFloat<f64>,
    z: UniformFloat<f64>,
}

impl UniformSampler for UniformVec {
    type X = Vec;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self {
            x: UniformFloat::new(low.borrow().x, high.borrow().x),
            y: UniformFloat::new(low.borrow().y, high.borrow().y),
            z: UniformFloat::new(low.borrow().z, high.borrow().z),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformSampler::new(low, high)
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
            z: self.z.sample(rng),
        }
    }
}

impl SampleUniform for Vec {
    type Sampler = UniformVec;
}
