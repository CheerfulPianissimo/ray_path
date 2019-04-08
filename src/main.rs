mod graphics;
mod shapes;

use self::graphics::*;
use self::shapes::*;

fn main() {
    let (w,h)=(100,100);
    let sphere=Sphere::new(Point3D::new(50.0,50.0,50.0),10.0,Material::new(RGBColor::new(1.0,1.0,1.0)));
    let plane=Plane::new(Point3D::origin(),Normal3D::new(0.0,1.0,-0.1),Material::new(RGBColor::new(1.0,1.0,1.0)));
    for y in 0..w{
        for x in 0..h{
            let ray=Ray::new(Point3D::new(x as f64,y as f64,100.0),Vector3D::new(0.0,0.0,-1.0));
            if sphere.check_hit(&ray).is_some()||plane.check_hit(&ray).is_some(){
                print!("{}",1);
            }else {
                print!(" ");
            }
        }
        println!();
    }
}
