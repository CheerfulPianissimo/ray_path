mod graphics;
mod shapes;
mod tracer;

use self::graphics::*;
use self::shapes::*;
use self::tracer::*;

fn main() {
    let metallic = MetallicMaterial::new(RGBColor::new(0.5, 0.5, 0.5),0.2);
    let diffuse1 = LambertianMaterial::new(RGBColor::new(0.3, 0.2, 0.6));
    let diffuse2 = LambertianMaterial::new(RGBColor::new(1.7, 1.7, 0.3));
    {
        let sphere = Sphere::new(Point3D::new(-2.0, -1.0, 0.0), 1.0,
                                 &metallic);
        let sphere2 = Sphere::new(Point3D::new(0.0, 1.0, -0.0), 1.0,
                                  &metallic);
        let sphere3 = Sphere::new(Point3D::new(-0.0, -1.0, 0.0), 1.0,
                                  &diffuse1);
        let sphere4 = Sphere::new(Point3D::new(0.0, -42.0, 0.0), 40.0,
                                  &diffuse2);
        let disc = ThinDisc::new(Point3D::new(3.0, 0.0, -2.0), 2.0,
                                 Normal3D::new(-1.0, -0.3, 0.8), &metallic);
        let plane = Plane::new(Point3D::new(0.0, -2.0, 0.0),
                               Normal3D::new(0.0, 1.0, 0.0),
                               &metallic);

        let mut world = World::new(ViewPlane::new(800, 600, 1.0 / 200.0),
                                   RGBColor::new(0.0, 0.0, 0.0));
        world.get_objects_mut().push(Box::new(sphere));
        world.get_objects_mut().push(Box::new(plane));
        world.get_objects_mut().push(Box::new(sphere2));
        world.get_objects_mut().push(Box::new(sphere3));
        //world.get_objects_mut().push(Box::new(sphere4));
        world.get_objects_mut().push(Box::new(disc));

        let tracer = SimpleTracer::new();

        tracer.render(&world).save("./img.jpeg").unwrap();
    }
}
