use std::ops::{Add,Neg,Sub,Mul,BitXor};

#[derive(Debug,Copy,Clone)]
pub struct Point3D{
    x:f64,
    y:f64,
    z:f64
}

impl Point3D{
    pub fn new(x:f64,y:f64,z:f64)->Point3D{
        Point3D{x,y,z}
    }

    pub fn origin()->Point3D{
        Self::new(0.0,0.0,0.0)
    }
}

impl Sub for Point3D{
    type Output=Vector3D;

    fn sub(self, other: Self) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Normal3D{
    x:f64,
    y:f64,
    z:f64,
}

impl Normal3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Normal3D { x, y, z }
    }
}

impl Mul for Normal3D{
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x*other.x+self.y*other.y+self.z*other.z
    }
}

impl Mul<Vector3D> for Normal3D{
    type Output = f64;

    fn mul(self, other: Vector3D) -> f64 {
        self.x*other.x+self.y*other.y+self.z*other.z
    }
}


#[derive(Debug,Copy,Clone)]
pub struct Vector3D{
    x:f64,
    y:f64,
    z:f64,
}


impl Vector3D{
    pub fn new(x:f64,y:f64,z:f64)->Vector3D{
        Vector3D{x,y,z}
    }

    pub fn null()->Vector3D{
        Self::new(0.0,0.0,0.0)
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


impl Neg for Vector3D{
    type Output=Self;

    fn neg(self)->Self{
        Self{
            x: -self.x ,
            y: -self.y,
            z: -self.z,
        }
    }
}
///Dot Product
impl Mul for Vector3D{
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x*other.x+self.y*other.y+self.z*other.z
    }
}

///Cross Product
impl BitXor for Vector3D{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.y*other.z-self.z*other.y,
            y: -self.x*other.z+self.z*other.x,
            z: self.x*other.y-self.y*other.x,
        }
    }
}

impl Mul<Normal3D> for Vector3D{
    type Output = f64;

    fn mul(self, other: Normal3D) -> f64 {
        self.x*other.x+self.y*other.y+self.z*other.z
    }
}

pub struct RGBColor{
    r:f64,
    g:f64,
    b:f64
}

impl RGBColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGBColor { r, g, b }
    }

    pub fn r_in_8_bit(&self)->u8{
        (self.r*255.0) as u8
    }

    pub fn g_in_8_bit(&self)->u8{
        (self.g*255.0) as u8
    }

    pub fn b_in_8_bit(&self)->u8{
        (self.b*255.0) as u8
    }
}