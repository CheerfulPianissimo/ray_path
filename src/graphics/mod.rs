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
        let new_dir =reflect(&ray_in,hit_info.get_normal());
        let scattered = new_dir + random_unit_vec * self.fuzziness;
        return (
            Ray::new(hit_point.clone(), scattered.normalize()),
            self.albedo,
        );
    }
}

fn reflect(ray_in:&Ray,normal:&Normal3D)->Vector3D{
    let ray_in_d=ray_in.d.normalize();
    ray_in_d - Vector3D::from(normal) * (ray_in_d * Vector3D::from(normal)) * 2.0
}

fn refract1(ray_in:&Ray,normal:&Normal3D,refractive_index:f64)->Option<Vector3D>{
    let ray_in_d=ray_in.d.normalize();
    let discriminant=
        1.0-refractive_index.powi(2)*(1.0-(normal*ray_in_d).powi(2));
    if discriminant<0.0{
        return None;
    }else{
        let discriminant_sqrt=discriminant.sqrt();
        let normal_vec= Vector3D::from(normal);
        /*let vec_out=
            ray_in_d*refractive_index + normal_vec*discriminant_sqrt
                - normal_vec*(normal_vec*ray_in_d)*refractive_index;*/
        let vec_out=
            (ray_in_d-Vector3D::from(normal*(normal*ray_in_d)))*refractive_index
                + normal_vec*discriminant_sqrt;
        //let vec_out=Vector3D::new(-vec_out.x(),-vec_out.y(),-vec_out.z());
        Some(vec_out)
    }
}

pub fn refract2(ray_in:&Ray,normal:&Normal3D,refractive_index:f64)->Option<Vector3D> {
    //https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    let i = ray_in.d.normalize();
    let n=Vector3D::from(normal);
    let cosi=(i*n)*-1.0;
    let sinsqr_t=refractive_index.powi(2)*(1.0-cosi.powi(2));
    let discr=1.0-sinsqr_t.powi(2);
    if discr<0.0{
        None
    }else{
        let t=i*refractive_index+n*(refractive_index*cosi-discr.sqrt());
        Some(t)
    }
}

fn schlick_reflectivity(cosine:f64,refractive_index:f64)->f64{
    let r0=((1.0-refractive_index)/(1.0+refractive_index)).powi(2);
    r0+(1.0-r0)*((1.0-cosine).powi(5))
}

pub struct Dielectric{
    refractive_index:f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric{
    fn process(&self, ray_in: &Ray, hitinfo: &HitInfo) -> (Ray, RGBColor) {
        let ray_in_d=ray_in.d.normalize();
        let normal=hitinfo.get_normal();
        let (surface_normal,refractive_index,cosine)=
            if ray_in_d*Vector3D::from(normal)>0.0{
                (normal*-1.0,self.refractive_index,
                    self.refractive_index*(ray_in_d*Vector3D::from(normal)))
            }else{
                (normal.clone(),1.0/self.refractive_index,
                 (ray_in_d*Vector3D::from(normal))*-1.0)
            };

        let attenuation=RGBColor::new(1.0,1.0,1.0);
        let ray_out_d=
            match refract2(&ray_in,&surface_normal,refractive_index){
                Some(result)=>{
                    let reflectivity=schlick_reflectivity(cosine,self.refractive_index);
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0.0,1.0)<reflectivity{
                        reflect(&ray_in,&surface_normal)
                    }else {
                        result
                    }
                },
                None=>{reflect(&ray_in,&surface_normal)}
            };
        (Ray::new(hitinfo.get_hitpoint().clone(),ray_out_d),attenuation)
    }
}