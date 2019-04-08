mod graphics;
mod shapes;

use self::graphics::*;
use self::shapes::*;

fn main() {
    let vec1=Vector3D::new(0.0,0.0,1.0);
    let vec2=Vector3D::new(1.0,0.0,0.0);
    println!("{:?}",vec1^vec2);
    let plane=Plane::new(Point3D::new(0.0,1.0,-1.0),Normal3D::new(0.0,1.0,0.0),
                         Material::new(RGBColor::new(1.0,1.0,1.0)));
    let ray=Ray::new(Point3D::new(0.0,0.0,0.0),Vector3D::new(0.0,0.0,-1.0));
    println!("{:?}",plane.check_hit(&ray));
    let sphere=Sphere::new(Point3D::origin(),1.0,Material::new(RGBColor::new(1.0,1.0,1.0)));
    let ray2=Ray::new(Point3D::new(-1.0,1.0,0.0),Vector3D::new(1.0,0.0,0.0));
    println!("{:?}",sphere.check_hit(&ray2));
}
