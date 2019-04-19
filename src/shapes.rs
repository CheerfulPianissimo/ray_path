use crate::graphics::*;

const K_EPSILON:f64=0.00001;

pub struct Plane<'a>{
        ///A point on the plane
        a:Point3D,
        ///A Normal to the plane
        n:Normal3D,
        material:&'a Material,
}

impl<'a> Plane<'a>{
        pub fn new(a: Point3D, n: Normal3D,material:&'a Material) -> Self {
                Plane { a, n ,material}
        }
}

impl<'a> GeometricObject for Plane<'a>{
        fn check_hit(&self,ray:&Ray)->Option<HitInfo>{
                let denominator=ray.d*self.n;
                if denominator==0.0 { //Ray is parallel to plane
                        return Option::None;
                }
                let t=((self.a-ray.o)*self.n)/denominator; //See https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
                if t> K_EPSILON {
                        Some(HitInfo::new(t,self.n.clone(),ray.get_point_at(t)))
                }else {
                        Option::None
                }
        }

        fn get_material(&self) -> &Material {
                self.material
        }
}

pub struct Sphere<'a>{
        ///Center of sphere
        c:Point3D,
        ///Radius of sphere
        r:f64,
        material:&'a Material
}

impl<'a> Sphere<'a> {
        pub fn new(c: Point3D, r: f64, material: &'a Material) -> Self {
                Sphere { c, r, material }
        }
}

impl<'a> GeometricObject for Sphere<'a> {
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
                                return Some(HitInfo::new(t1,
                                                         Normal3D::from(normal), hitpoint));
                        }
                        let t2 = (-b + discriminant.sqrt()) / 2.0 * a; //larger

                        if t2 > K_EPSILON {
                                let hitpoint = ray.get_point_at(t2);
                                let normal = hitpoint - self.c;
                                return Some(HitInfo::new(t2,
                                                         Normal3D::from(normal),
                                                         hitpoint));
                        } else {
                                //Both t1 and t2 are negative or 0
                                return None;
                        }
                }
        }

        fn get_material(&self) -> &Material {
                self.material
        }
}

