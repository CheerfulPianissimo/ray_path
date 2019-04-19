mod graphics;
mod shapes;
mod tracer;

use self::graphics::*;
use self::shapes::*;
use self::tracer::*;

fn main() {
        let material=LambertianMaterial::new(0.5);
    {
        let sphere = Sphere::new(Point3D::new(-1.0, -1.0, 0.0), 1.0,
                                 &material);

        let sphere2 = Sphere::new(Point3D::new(000.0, -42.0, 0.0), 40.0,
                                  &material);
        let sphere3 = Sphere::new(Point3D::new(1.0, -1.0, 0.0), 1.0,
                                  &material);
        let plane = Plane::new(Point3D::new(0.0, -2.0, 0.0),
                               Normal3D::new(0.0, 1.0, 0.0),
                               &material);

        let mut world = World::new(ViewPlane::new(800, 600, 1.0 / 150.0),
                                   RGBColor::new(1.0, 1.0, 1.0));
        world.get_objects_mut().push(Box::new(sphere));
        //world.get_objects_mut().push(Box::new(plane));
        world.get_objects_mut().push(Box::new(sphere2));
        world.get_objects_mut().push(Box::new(sphere3));
        let tracer = SimpleTracer::new();

        tracer.render(&world).save("./img.jpeg").unwrap();
    }
}
