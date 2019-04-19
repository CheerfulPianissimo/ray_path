use super::{RGBColor,Material};
use image::{GenericImage, DynamicImage, Pixel, RgbaImage};
use crate::graphics::{Point3D, Vector3D, Ray, GeometricObject, World, ViewPlane, HitInfo, Normal3D};
use rand::{Rng};
use core::num::FpCategory::Normal;

pub struct SimpleTracer;

impl SimpleTracer {
    pub fn new() -> Self {
        SimpleTracer {}
    }

    pub fn render(&self, world: &World) -> DynamicImage {
        let z_plane = 5.0;
        let mut img =
            DynamicImage::new_rgb8(world.get_view_plane().get_hres(), world.get_view_plane().get_vres());

        let vres=world.get_view_plane().get_vres();
        let hres=world.get_view_plane().get_hres();
        let pixel_size=world.get_view_plane().get_pixel_size();
        let mut rng = rand::thread_rng();

        for y in -((vres/ 2) as i32)..(vres/ 2) as i32 {
            println!("Completed: {}%",((y+vres as i32/2)*100)/vres as i32);
            for x in -((hres/ 2) as i32)..(hres/ 2) as i32 {
                let mut avg_color=RGBColor::new(0.0,0.0,0.0);
                let samples:f64=49.0;
                let samples_sqrt=samples.sqrt() as u32;
                let sub_pixel_size=pixel_size/samples_sqrt as f64;
                for p in 0..samples_sqrt{
                    for q in 0..samples_sqrt {
                        let in_world_x = x as f64 * pixel_size +sub_pixel_size*p as f64+
                                            rng.gen_range(0.0, sub_pixel_size);
                        let in_world_y = y as f64 * pixel_size+sub_pixel_size*q as f64+
                            rng.gen_range(0.0, sub_pixel_size);

                        let ray_direction = Point3D::new(in_world_x, in_world_y, z_plane) -
                            Point3D::new(0.0, 0.0, 10.0);
                        let ray = Ray::new(Point3D::new(in_world_x, in_world_y, z_plane),
                                           //Vector3D::new(0.0,0.0,-1.0));
                                           ray_direction.normalize());
                        let pixel_color = self.trace_ray(&ray, &world,500000);
                        avg_color.r += pixel_color.r;
                        avg_color.g += pixel_color.g;
                        avg_color.b += pixel_color.b;
                    }
                }
                avg_color.r/=samples;avg_color.g/=samples;avg_color.b/=samples;
                avg_color.r=avg_color.r.sqrt();avg_color.g=avg_color.g.sqrt();avg_color.b=avg_color.b.sqrt();
                let img_x = (x + (hres / 2) as i32) as u32;
                let img_y = ((vres / 2) as i32 - y - 1) as u32;

                img.put_pixel(img_x, img_y, image::Rgba::from_channels(
                    avg_color.r_in_8_bit(),avg_color.g_in_8_bit(),avg_color.b_in_8_bit(), 255));
            }
        }
        img
    }

    fn trace_ray(&self,ray:&Ray,world:&World,depth:u32)->RGBColor{
        let (mut min_hitinfo,mut material)=(None,None);
        let mut min=std::f64::MAX;
        for object in world.get_objects(){
            match object.check_hit(ray){
                Some(hitinfo)=>{
                    if hitinfo.get_tmin()<min{
                        min=hitinfo.get_tmin();
                        min_hitinfo=Some(hitinfo);
                        material=Some(object.get_material());
                    }
                },
                None=>{}
            }
        }
        match min_hitinfo {
            Some(hit_info)=> {
                if depth==0 {
                    let normal = hit_info.get_normal().normalize();
                    RGBColor::new((normal.x() + 1.0) / 2.0,
                                  (normal.y() + 1.0) / 2.0,
                                  (normal.z() + 1.0) / 2.0)
                    //return material.unwrap().get_color().clone()
                }else {
                    let (ray_out,attenuation)=material.unwrap().process(ray,&hit_info);
                    return self.trace_ray(&ray_out,
                                          &world,depth-1)*attenuation;
                }
            },
            None=>{
                //assert_eq!(ray.d.magnitude_sqr(),1.0);
                let y=ray.d.y();
                let t=(y+1.0)*0.5;
                //assert!(t<1.0&&t>0.0);
                let blue=Vector3D::new(0.4,0.4,1.0);
                let white=Vector3D::new(1.0,1.0,1.0);
                let colorvec=blue*(1.0-t)+white*t;
                RGBColor::from(colorvec)
                //world.get_bg_color().clone()
            }
        }

    }
}