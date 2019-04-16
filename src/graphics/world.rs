use super::{RGBColor};
use crate::graphics::{Ray, GeometricObject};
use crate::graphics::{Point3D};
use crate::shapes::Sphere;

pub struct ViewPlane{
    hres:u32,
    vres:u32,
    ///Pixel size: Number of  in-world units corresponding to a pixel
    s:f64,
}

impl ViewPlane {
    pub fn new(hres: u32, vres: u32, s: f64) -> Self {
        ViewPlane { hres, vres, s }
    }

    pub fn get_vres(&self)->u32{
        self.vres
    }

    pub fn get_hres(&self)->u32{
        self.hres
    }

    ///Pixel size: Number of  in-world units corresponding to a pixel
    pub fn get_pixel_size(&self)->f64{
        self.s
    }
}

pub struct World{
    v_plane:ViewPlane,
    bg_color:RGBColor,
    objects:Vec<Box<dyn GeometricObject>>
}

impl World {
    pub fn new(v_plane: ViewPlane, bg_color: RGBColor) -> Self {
        World {v_plane , bg_color ,objects:Vec::new()}
    }

    pub fn get_objects_mut(&mut self)->&mut Vec<Box<dyn GeometricObject>>{
        &mut self.objects
    }

    pub fn get_objects(&self)->&Vec<Box<dyn GeometricObject>>{
        &self.objects
    }

    pub fn get_view_plane(&self)->&ViewPlane{
        &self.v_plane
    }

    pub fn get_bg_color(&self)->&RGBColor{
        &self.bg_color
    }
}

