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
}

pub trait Hittable{
    fn check_hit(&self,ray:&Ray)->Option<HitInfo>;
}

#[derive(Debug)]
pub struct HitInfo{
    ///Lowest value of ray parameter t which intersects Hittable object
    pub tmin:f64,
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

pub trait HasMaterial{
    fn get_material(&self)->&Material;
}
