mod data_structures;
mod world;


pub use self::data_structures::*;
pub use self::world::*;

///Infinite Ray represented by p=o+td
pub struct Ray{
    ///Origin of ray
    pub o:Point3D,
    ///Direction in which it points
    pub d:Vector3D,
}

impl Ray{
    pub fn new(o:Point3D,d:Vector3D)->Ray{
        Ray{o,d}
    }

    pub fn get_point_at(&self,t:f64)->Point3D{
        self.o+self.d*t
    }
}

pub trait GeometricObject{
    fn check_hit(&self,ray:&Ray)->Option<HitInfo>;

    fn get_material(&self)->&Material;
}

#[derive(Debug)]
pub struct HitInfo{
    ///Lowest value of ray parameter t which intersects Hittable object
    tmin:f64,
    normal:Normal3D
}

impl HitInfo {
    pub fn new(tmin: f64, normal: Normal3D) -> Self {
        HitInfo { tmin, normal }
    }

    pub fn get_tmin(&self)->f64{self.tmin}

    pub fn get_normal(&self)->&Normal3D{
        &self.normal
    }
}


pub struct Material{
    color:RGBColor
}

impl Material {
    pub fn new(color: RGBColor) -> Self {
        Material { color }
    }

    pub fn get_color(&self)->&RGBColor{
        &self.color
    }
}

