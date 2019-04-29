mod graphics;
mod shapes;
mod tracer;

use self::graphics::*;
use self::shapes::*;
use self::tracer::*;
use std::rc::Rc;
use image::{DynamicImage, GenericImage, Pixel, RgbaImage};
use std::sync::mpsc::Sender;

fn main() {
    let (sender, recv) = std::sync::mpsc::channel();
    let (hres, vres, s,samples) = (1366, 768, 1.0 / (300.0),4);
    let sender_clone = sender.clone();
    std::thread::spawn(move || {
        setup_and_run(sender_clone, hres, vres, s, samples);
    });
    let mut img = DynamicImage::new_rgb8(hres, vres);
    let mut total_array=vec![vec![RGBColor::new(0.0,0.0,0.0)
                                  ; vres as usize];hres as usize];
    loop{
        match recv.recv().unwrap(){
            PixelInfo::Pixel(x, y, color,samples_rendered)=>{
                total_array[x as usize][y as usize].r+=color.r;
                total_array[x as usize][y as usize].g+=color.g;
                total_array[x as usize][y as usize].b+=color.b;
                let pixel=total_array[x as usize][y as usize];
                //println!("{}",samples_rendered);
                let new_pixel=RGBColor::new(pixel.r/samples_rendered as f64,
                                            pixel.g/samples_rendered as f64,pixel.b/samples_rendered as f64);
                img.put_pixel(x, y, image::Rgba::from_channels(
                    new_pixel.r_in_8_bit(),
                    new_pixel.g_in_8_bit(),
                    new_pixel.b_in_8_bit(),
                    255,
                ));
            },
            PixelInfo::SampleComplete(samples_rendered)=>{
                println!("Samples rendered: {} ",samples_rendered);
                img.save(format!("./img.jpeg")).unwrap();

            },
            PixelInfo::End=>{
                break;
            }
        }
    }
}

fn setup_and_run(sender: Sender<(PixelInfo)>,hres:u32,vres:u32,s:f64,samples:u32) {
    let world=get_world(hres, vres, s, samples);

    let tracer = SimpleTracer::new();
    tracer.render(&world,sender);
}


fn get_world(hres:u32,vres:u32,s:f64,samples:u32)->World{
    let metallic1 = Rc::new(MetallicMaterial::new(RGBColor::new(0.5, 0.5, 0.5), 0.5));
    let metallic2 = Rc::new(MetallicMaterial::new(RGBColor::new(1.0, 1.0, 1.0), 0.0));
    let diffuse1 = Rc::new(LambertianMaterial::new(RGBColor::new(0.3, 0.2, 0.6)));
    let diffuse2 = Rc::new(LambertianMaterial::new(RGBColor::new(0.9, 0.5, 0.0)));
    let dielectric1 = Rc::new(Dielectric::new(1.3));
    let emit1 = Rc::new(DiffuseLight::new(RGBColor::new(1.0, 1.0, 1.0)));


    let sphere1 = Sphere::new(Point3D::new(-2.0, -1.0, 0.0), 1.0, metallic2.clone());
    let sphere2 = Sphere::new(Point3D::new(2.0, -1.0, 0.0), 0.3, emit1);
    let sphere3 = Sphere::new(Point3D::new(-0.0, -1.0, 0.0), 1.0, dielectric1);
    let sphere4 = Sphere::new(Point3D::new(0.0, -42.0, 0.0), 40.0, diffuse2);
    let disc = ThinDisc::new(
        Point3D::new(3.0, 0.0, -1.2),
        2.0,
        Normal3D::new(-1.0, -0.3, 0.8),
        //Normal3D::new(0.0,0.0,1.0),
        metallic2.clone(),
    );
    let plane = Plane::new(
        Point3D::new(0.0, 2.0, 0.0),
        Normal3D::new(0.0, 1.0, 0.0),
        metallic2.clone(),
    );

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