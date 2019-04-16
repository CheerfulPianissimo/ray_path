mod graphics;
mod shapes;
mod tracer;

use self::graphics::*;
use self::shapes::*;
use self::tracer::*;

fn main() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, -50.0), 10.0,
                                 Material::new(RGBColor::new(1.0, 0.0, 0.0)));
        let sphere2 = Sphere::new(Point3D::new(000.0, -110.0, -20.0), 100.0,
                                  Material::new(RGBColor::new(0.0, 1.0, 0.0)));
        let plane = Plane::new(Point3D::origin(), Normal3D::new(0.0, 0.0, 1.0),
                               Material::new(RGBColor::new(0.0, 1.0, 1.0)));

        let mut world = World::new(ViewPlane::new(500, 500, 1.0/20.0), RGBColor::new(0.0, 0.0, 0.0));
        world.get_objects_mut().push(Box::new(sphere));
        // world.get_objects_mut().push(Box::new(plane));
        world.get_objects_mut().push(Box::new(sphere2));
        let tracer = SimpleTracer::new();

        tracer.render(&world).save("./img.jpeg").unwrap();
}
