use std::ops::{Add, BitXor, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn origin() -> Point3D {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Sub for Point3D {
    type Output = Vector3D;

    fn sub(self, other: Self) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, other: Vector3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Normal3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Normal3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Normal3D { x, y, z }
    }

    pub fn normalize(&self) -> Normal3D {
        let dist_sqr=(self.x * self.x + self.y * self.y + self.z * self.z);
        if dist_sqr>0.999999&&dist_sqr<1.000001{ //Already a normalized normal
            return self.clone();
        }
        let inverse = 1.0 / (dist_sqr.sqrt());
        Normal3D {
            x: self.x * inverse,
            y: self.y * inverse,
            z: self.z * inverse,
        }
    }

    pub fn magnitude_sqr(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Mul for Normal3D {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<Vector3D> for Normal3D {
    type Output = f64;

    fn mul(self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<Vector3D> for &Normal3D {
    type Output = f64;

    fn mul(self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

//Scaling
impl Mul<f64> for Normal3D {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Normal3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for &Normal3D {
    type Output = Normal3D;

    fn mul(self, other: f64) -> Normal3D {
        Normal3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl From<Vector3D> for Normal3D {
    fn from(vec3: Vector3D) -> Self {
        Normal3D {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn null() -> Vector3D {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self) -> Vector3D {
        let dist_sqr=(self.x * self.x + self.y * self.y + self.z * self.z);
        if dist_sqr>0.999999&&dist_sqr<1.000001{ //Already a normalized vector
            return self.clone();
        }
        let inverse = 1.0 / (dist_sqr.sqrt());
        Vector3D {
            x: self.x * inverse,
            y: self.y * inverse,
            z: self.z * inverse,
        }
    }

    pub fn magnitude_sqr(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

///Dot Product
impl Mul for Vector3D {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

///Cross Product
impl BitXor for Vector3D {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -self.x * other.z + self.z * other.x,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

//Scaling
impl Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vector3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Normal3D> for Vector3D {
    type Output = f64;

    fn mul(self, other: Normal3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl From<Normal3D> for Vector3D {
    fn from(norm: Normal3D) -> Self {
        Vector3D {
            x: norm.x,
            y: norm.y,
            z: norm.z,
        }
    }
}

impl From<&Normal3D> for Vector3D {
    fn from(norm: &Normal3D) -> Self {
        Vector3D {
            x: norm.x,
            y: norm.y,
            z: norm.z,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGBColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGBColor { r, g, b }
    }

    pub fn r_in_8_bit(&self) -> u8 {
        (((self.r * 255.0) as u32)%256) as u8
    }

    pub fn g_in_8_bit(&self) -> u8 {
        (((self.g * 255.0) as u32)%256) as u8
    }

    pub fn b_in_8_bit(&self) -> u8 {
        (((self.b * 255.0) as u32)%256) as u8
    }
}

impl Mul<f64> for RGBColor {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        RGBColor {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl From<Vector3D> for RGBColor {
    fn from(vec3: Vector3D) -> Self {
        RGBColor {
            r: vec3.x,
            g: vec3.y,
            b: vec3.z,
        }
    }
}

impl Mul for RGBColor {
    type Output = RGBColor;

    fn mul(self, other: Self) -> Self {
        RGBColor {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}


impl Add for RGBColor {
    type Output = RGBColor;

    fn add(self, other: Self) -> Self {
        RGBColor {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}