mod graphics;
mod shapes;
mod tracer;

use self::graphics::*;
use self::shapes::*;
use self::tracer::*;
use std::rc::Rc;

fn main() {
    let tracer = SimpleTracer::new();
    for t in 0..1{
        println!("Completed frames {}",t+1);
        tracer.render(Box::new(get_world),&format!("./render/img{:06}.jpeg",t),t as f64/5.0);
    }
}


fn get_world(t:f64)->World{
    let (hres, vres, s,samples) = (800, 800, 1.0 / (200.0+t/5.0),64);
    let metallic1 = Rc::new(MetallicMaterial::new(RGBColor::new(0.5, 0.5, 0.5), 0.5*t.sin()+0.5));
    let metallic2 = Rc::new(MetallicMaterial::new(RGBColor::new(1.0, 1.0, 1.0), 0.0));
    let diffuse1 = Rc::new(LambertianMaterial::new(RGBColor::new(0.3, 0.2, 0.6)));
    let diffuse2 = Rc::new(LambertianMaterial::new(RGBColor::new(0.9, 0.5, 0.0)));
    let dielectric1 = Rc::new(Dielectric::new(1.3));
    //let emit1 = Rc::new(DiffuseLight::new(RGBColor::new(1.0, 1.0, 1.0)));


    let sphere1 = Sphere::new(Point3D::new(41.0*(t/5.0).cos(), -42.0+41.0*(t/5.0).sin(), 0.0), 1.0, metallic2.clone());
    let sphere2 = Sphere::new(Point3D::new(2.0, -1.0*t.sin(), 0.0), 1.0, diffuse1.clone());
    let sphere3 = Sphere::new(Point3D::new(-41.0*(t/5.0).cos(), -42.0+41.0*(t/5.0).sin(), t.sin()), 1.0, dielectric1);
    let sphere4 = Sphere::new(Point3D::new(0.0, -42.0, 0.0), 40.0, diffuse2.clone());
    let disc = ThinDisc::new(
        Point3D::new(3.0, 0.0, -1.2),
        2.0,
        Normal3D::new(-1.0, -0.3*t.sin(), 0.8),
        //Normal3D::new(0.0,0.0,1.0),
        metallic1.clone()
    );
    /*let plane = Plane::new(
        Point3D::new(0.0, 2.0, -5.0),
        Normal3D::new(0.0, 1.0, 1.0),
        metallic2.clone(),
    );*/

    let mut world = World::new(
        ViewPlane::new(hres, vres, s, samples),
        RGBColor::new(0.0, 0.0, 0.0),
    );
    world.get_objects_mut().push(Box::new(sphere1));
    //world.get_objects_mut().push(Box::new(plane));
    world.get_objects_mut().push(Box::new(sphere2));
    world.get_objects_mut().push(Box::new(sphere3));
    world.get_objects_mut().push(Box::new(sphere4));
    world.get_objects_mut().push(Box::new(disc));
    world
}
