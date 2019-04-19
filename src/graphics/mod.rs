mod data_structures;
mod world;


pub use self::data_structures::*;
pub use self::world::*;
use rand::{thread_rng,Rng};

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
    normal:Normal3D,
    hitpoint:Point3D
}

impl HitInfo {
    pub fn new(tmin: f64, normal: Normal3D, hitpoint: Point3D) -> Self {
        HitInfo { tmin, normal, hitpoint }
    }

    pub fn get_tmin(&self)->f64{self.tmin}

    pub fn get_normal(&self)->&Normal3D{
        &self.normal
    }

    pub fn get_hitpoint(&self)->&Point3D{
        &self.hitpoint
    }
}

pub trait Material{
    ///ray-normalised incoming ray
    ///returns outgoing normalized ray and attenuation
    fn process(&self,ray_in:&Ray,hitinfo:&HitInfo)->(Ray,f64);
}

pub struct LambertianMaterial{
    albedo:f64,
}

impl LambertianMaterial {
    pub fn new(albedo:f64) -> Self {
        LambertianMaterial { albedo }
    }

}

impl Material for LambertianMaterial{
    fn process(&self,ray_in:&Ray,hit_info:&HitInfo)->(Ray,f64){
        let hit_point=hit_info.get_hitpoint();
        let mut rng = rand::thread_rng();
        let random_unit_vec=Vector3D::new(rng.gen_range(-1.0,1.0),
                                          rng.gen_range(-1.0,1.0),
                                          rng.gen_range(-1.0,1.0) ).normalize();
        let ray_out=
            (Vector3D::from(hit_info.get_normal().normalize())+
                random_unit_vec).normalize();
        (Ray::new(hit_point.clone(),ray_out),self.albedo)
    }
}

