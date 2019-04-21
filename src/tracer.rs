use super::{Material, RGBColor};
use crate::graphics::{
    GeometricObject, HitInfo, Normal3D, Point3D, Ray, Vector3D, ViewPlane, World,
};
use image::{DynamicImage, GenericImage, Pixel, RgbaImage};
use rand::Rng;
use rayon::prelude::*;
use std::sync::mpsc::Sender;

pub enum PixelInfo {
    ///x,y,pixel color, sample_no
    Pixel(u32, u32, RGBColor,u32),
    SampleComplete(u32),
    End,
}

pub struct SimpleTracer;

impl SimpleTracer {
    pub fn new() -> Self {
        SimpleTracer {}
    }

    pub fn render(&self, world: World)->DynamicImage{
        let  world = World::new(
            ViewPlane::new(600, 600, 1.0/200.0, 169),
            RGBColor::new(0.0, 0.0, 0.0),
        );
        let (sender, recv) = std::sync::mpsc::channel();
        let num_cpu=2;
        let vres = world.get_view_plane().get_vres();
        let hres = world.get_view_plane().get_hres();
        let section_height=vres/num_cpu;
        let lowest_section_y=-(vres as i32/2);
        let remaining_section_height=vres%num_cpu;

        let world_ref=std::sync::Arc::new(world);

        for i in 0..num_cpu{
            let sender_clone = sender.clone();
            let ref_clone=world_ref.clone();
            std::thread::spawn(move ||{
                SimpleTracer::render_image_section(&ref_clone,sender_clone,lowest_section_y+(i*section_height) as i32,
                                                                lowest_section_y+((i+1)*section_height) as i32);
            });
        }

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
        img
    }

    ///lowY-in world co-ordinates of row from which to render
    ///highY-in world co-ordinates of row upto which rendering should occur
    fn render_image_section( world: &World, sender: Sender<(PixelInfo)>,
                  lowY:i32,highY:i32) {
        let z_plane = 5.0;
        let samples = world.get_view_plane().get_samples();
        let samples_sqrt = (samples as f64).sqrt() as u32;

        let vres = world.get_view_plane().get_vres();
        let hres = world.get_view_plane().get_hres();
        let pixel_size = world.get_view_plane().get_pixel_size();
        let mut rng = rand::thread_rng();
        let (mut sub_x,mut sub_y)=(0,0);
        let sub_pixel_size = pixel_size / samples_sqrt as f64;
        let mut samples_rendered=0;

        for sub_y in 0..samples_sqrt {
            for sub_x in 0..samples_sqrt{
                samples_rendered+=1;
                for y in lowY..highY {
                    /*let completion = ((y + vres as i32 / 2) * 100) / vres as i32;
                    if completion % 10 == 0 {
                        //print!("{}% ",completion);
                    }*/
                    for x in -((hres / 2) as i32)..(hres / 2) as i32 {
                        let in_world_x = x as f64 * pixel_size
                            + sub_pixel_size * sub_x as f64
                            + rng.gen_range(0.0, sub_pixel_size);
                        let in_world_y = y as f64 * pixel_size
                            + sub_pixel_size * sub_y as f64
                            + rng.gen_range(0.0, sub_pixel_size);

                        let ray_direction = Point3D::new(in_world_x, in_world_y, z_plane)
                            - Point3D::new(0.0, 0.0, 10.0);
                        let ray = Ray::new(
                            Point3D::new(in_world_x, in_world_y, z_plane),
                            //Vector3D::new(0.0,0.0,-1.0));
                            ray_direction.normalize(),
                        );
                        let mut pixel_color = SimpleTracer::trace_ray(&ray, &world, 20);
                        pixel_color.r = pixel_color.r.sqrt();
                        pixel_color.g = pixel_color.g.sqrt();
                        pixel_color.b = pixel_color.b.sqrt();
                        assert!(pixel_color.r>=0.0);
                        let img_x = (x + (hres / 2) as i32) as u32;
                        let img_y = ((vres / 2) as i32 - y - 1) as u32;
                        sender
                            .send(PixelInfo::Pixel(
                                img_x,
                                img_y,
                                pixel_color,samples_rendered
                            ))
                            .unwrap();
                    }
                }
                sender.send(PixelInfo::SampleComplete(samples_rendered));
            }
        }

        sender.send(PixelInfo::End).unwrap();
    }

    fn trace_ray( ray: &Ray, world: &World, depth: u32) -> RGBColor {
        let (mut min_hitinfo, mut material) = (None, None);
        let mut min = std::f64::MAX;
        for object in world.get_objects() {
            match object.check_hit(ray) {
                Some(hitinfo) => {
                    if hitinfo.get_tmin() < min {
                        min = hitinfo.get_tmin();
                        min_hitinfo = Some(hitinfo);
                        material = Some(object.get_material());
                    }
                }
                None => {}
            }
        }
        match min_hitinfo {
            Some(hit_info) => {
                if depth == 0 {
                    let y = ray.d.y();
                    let t = (y + 1.0) * 0.5;
                    //assert!(t<1.0&&t>0.0);
                    let blue = Vector3D::new(0.4, 0.4, 1.0);
                    let white = Vector3D::new(1.0, 1.0, 1.0);
                    let colorvec = white * (1.0 - t) + blue * t;
                    RGBColor::from(colorvec)
                //return material.unwrap().get_color().clone()
                } else {
                    let (ray_out, attenuation) = material.unwrap().process(ray, &hit_info);
                    return SimpleTracer::trace_ray(&ray_out, &world, depth - 1) * attenuation;
                }
            }
            None => {
                //assert_eq!(ray.d.magnitude_sqr(),1.0);
                let y = ray.d.y();
                let t = (y + 1.0) * 0.5;
                //assert!(t<1.0&&t>0.0);
                let blue = Vector3D::new(0.4, 0.4, 1.0);
                let white = Vector3D::new(1.0, 1.0, 1.0);
                let colorvec = white * (1.0 - t) + blue * t;
                RGBColor::from(colorvec)
                //world.get_bg_color().clone()
            }
        }
    }
}
