use crate::graphics::*;

const K_EPSILON:f64=0.000001;

pub struct Plane{
        ///A point on the plane
        a:Point3D,
        ///A Normal to the plane
        n:Normal3D,
        material:Material,
}

impl Plane {
        pub fn new(a: Point3D, n: Normal3D,material:Material) -> Self {
                Plane { a, n ,material}
        }
}

impl GeometricObject for Plane{
        fn check_hit(&self,ray:&Ray)->Option<HitInfo>{
                let denominator=ray.d*self.n;
                if denominator==0.0 { //Ray is parallel to plane
                        return Option::None;
                }
                let t=((self.a-ray.o)*self.n)/denominator; //See https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
                if t> K_EPSILON {
                        Some(HitInfo::new(t,self.n.clone()))
                }else {
                        Option::None
                }
        }

        fn get_material(&self) -> &Material {
                &self.material
        }
}

pub struct Sphere{
        ///Center of sphere
        c:Point3D,
        ///Radius of sphere
        r:f64,
        material:Material
}

impl Sphere {
        pub fn new(c: Point3D, r: f64, material: Material) -> Self {
                Sphere { c, r, material }
        }
}

impl GeometricObject for Sphere{
        fn check_hit(&self,ray:&Ray)->Option<HitInfo>{
                let distance=ray.o-self.c;
                let a=ray.d*ray.d;
                let b=2.0*(distance*ray.d);
                let c=distance*distance- self.r*self.r;

                let discriminant=b*b-4.0*a*c;

                if discriminant<0.0{
                        return None;
                }else{
                        let t1=(-b-discriminant.sqrt())/2.0*a; //smaller

                        if t1>K_EPSILON{
                                let normal=ray.get_point_at(t1)-self.c;
                                return Some(HitInfo::new(t1, Normal3D::from(normal)));
                        }
                        let t2=(-b+discriminant.sqrt())/2.0*a; //larger

                        if t2>K_EPSILON{
                                let normal=ray.get_point_at(t2)-self.c;
                                return Some(HitInfo::new(t2,Normal3D::from(normal)));
                        }else {
                                //Both t1 and t2 are negative or 0
                                return None;
                        }
                }
        }

        fn get_material(&self) -> &Material {
                &self.material
        }
}

