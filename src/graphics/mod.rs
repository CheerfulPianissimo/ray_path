mod data_structures;
mod world;

pub use self::data_structures::*;
pub use self::world::*;
use rand::{thread_rng, Rng};

///Infinite Ray represented by p=o+td
pub struct Ray {
    ///Origin of ray
    pub o: Point3D,
    ///Direction in which it points
    pub d: Vector3D,
}

impl Ray {
    pub fn new(o: Point3D, d: Vector3D) -> Ray {
        Ray { o, d }
    }

    pub fn get_point_at(&self, t: f64) -> Point3D {
        self.o + self.d * t
    }
}

pub trait GeometricObject : Send+Sync{
    fn check_hit(&self, ray: &Ray) -> Option<HitInfo>;

    fn get_material(&self) -> &Material;
}

#[derive(Debug)]
pub struct HitInfo {
    ///Lowest value of ray parameter t which intersects Hittable object
    tmin: f64,
    ///The normalised normal
    normal: Normal3D,
    hitpoint: Point3D,
}

impl HitInfo {
    pub fn new(tmin: f64, normal: Normal3D, hitpoint: Point3D) -> Self {
        HitInfo {
            tmin,
            normal,
            hitpoint,
        }
    }

    pub fn get_tmin(&self) -> f64 {
        self.tmin
    }

    pub fn get_normal(&self) -> &Normal3D {
        &self.normal
    }

    pub fn get_hitpoint(&self) -> &Point3D {
        &self.hitpoint
    }
}

pub trait Material : Sync+Send {
    ///ray-normalised incoming ray
    ///returns outgoing normalized ray and attenuation
    fn process(&self, ray_in: &Ray, hitinfo: &HitInfo) -> (Ray, RGBColor);
}

pub struct LambertianMaterial {
    albedo: RGBColor,
}

impl LambertianMaterial {
    pub fn new(albedo: RGBColor) -> Self {
        LambertianMaterial { albedo }
    }
}

impl Material for LambertianMaterial {
    fn process(&self, _ray_in: &Ray, hit_info: &HitInfo) -> (Ray, RGBColor) {
        let hit_point = hit_info.get_hitpoint();
        let mut rng = rand::thread_rng();
        let random_unit_vec = Vector3D::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        )
        .normalize();
        let ray_out =
            (Vector3D::from(hit_info.get_normal().normalize()) + random_unit_vec).normalize();
        (Ray::new(hit_point.clone(), ray_out), self.albedo)
    }
}

pub struct MetallicMaterial {
    albedo: RGBColor,
    fuzziness: f64,
}

impl MetallicMaterial {
    pub fn new(albedo: RGBColor, fuzziness: f64) -> Self {
        MetallicMaterial { albedo, fuzziness }
    }
}

impl Material for MetallicMaterial {
    fn process(&self, ray_in: &Ray, hit_info: &HitInfo) -> (Ray, RGBColor) {
        let hit_point = hit_info.get_hitpoint();
        let mut rng = rand::thread_rng();
        let random_unit_vec = Vector3D::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        )
        .normalize();

        //Reflect
        let new_dir =
            ray_in.d - Vector3D::from(hit_info.normal) * (ray_in.d * hit_info.normal) * 2.0;
        let scattered = new_dir + random_unit_vec * self.fuzziness;
        (
            Ray::new(hit_point.clone(), scattered.normalize()),
            self.albedo,
        )
    }
}
