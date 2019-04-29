use crate::graphics::*;
use std::rc::Rc;
const K_EPSILON: f64 = 0.00001;

pub struct Plane {
    ///A point on the plane
    a: Point3D,
    ///A normalised Normal to the plane
    n: Normal3D,
    material: Rc<dyn Material>,
}

impl Plane {
    pub fn new(a: Point3D, n: Normal3D, material: Rc<dyn Material>) -> Self {
        Plane {
            a,
            n: n.normalize(),
            material,
        }
    }
}

impl GeometricObject for Plane {
    fn check_hit(&self, ray: &Ray) -> Option<HitInfo> {
        let denominator = ray.d * self.n;
        if denominator == 0.0 {
            //Ray is parallel to plane
            return Option::None;
        }
        let t = ((self.a - ray.o) * self.n) / denominator; //See https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        if t > K_EPSILON {
            Some(HitInfo::new(t, self.n.clone(), ray.get_point_at(t)))
        } else {
            Option::None
        }
    }

    fn get_material(&self) ->  Rc<dyn Material>{
        Rc::clone(&self.material)
    }
}

pub struct Sphere {
    ///Center of sphere
    c: Point3D,
    ///Radius of sphere
    r: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3D, r: f64, material: Rc<dyn Material>) -> Self {
        Sphere { c, r, material }
    }
}

impl GeometricObject for Sphere {
    fn check_hit(&self, ray: &Ray) -> Option<HitInfo> {
        let distance = ray.o - self.c;
        let a = ray.d * ray.d;
        let b = 2.0 * (distance * ray.d);
        let c = distance * distance - self.r * self.r;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / 2.0 * a; //smaller

            if t1 > K_EPSILON {
                let hitpoint = ray.get_point_at(t1);
                let normal = hitpoint - self.c;
                //dbg!(normal);
                return Some(HitInfo::new(
                    t1,
                    Normal3D::from(normal).normalize(),
                    hitpoint,
                ));
            }
            let t2 = (-b + discriminant.sqrt()) / 2.0 * a; //larger

            if t2 > K_EPSILON {
                let hitpoint = ray.get_point_at(t2);
                let normal = hitpoint - self.c;
                return Some(HitInfo::new(
                    t2,
                    Normal3D::from(normal).normalize(),
                    hitpoint,
                ));
            } else {
                //Both t1 and t2 are negative or 0
                return None;
            }
        }
    }

    fn get_material(&self) ->  Rc<dyn Material>{
        Rc::clone(&self.material)
    }
}

pub struct ThinDisc {
    ///Center of the disc
    c: Point3D,
    ///A normalised Normal to the disc
    n: Normal3D,
    ///Radius of disk
    r: f64,
    material: Rc<dyn Material>,
}

impl ThinDisc {
    pub fn new(c: Point3D, r: f64, n: Normal3D, material: Rc<dyn Material>) -> Self {
        ThinDisc {
            c,
            n: n.normalize(),
            r,
            material,
        }
    }
}

impl GeometricObject for ThinDisc {
    fn check_hit(&self, ray: &Ray) -> Option<HitInfo> {
        let plane = Plane::new(self.c, self.n, Rc::clone(&self.material));
        match plane.check_hit(ray) {
            Some(hitinfo) => {
                let hitpoint = *hitinfo.get_hitpoint();
                let distance_from_center_sqr = (hitpoint - self.c) * (hitpoint - self.c);
                if distance_from_center_sqr <= self.r * self.r {
                    Some(hitinfo)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn get_material(&self) ->  Rc<dyn Material>{
        Rc::clone(&self.material)
    }
}
