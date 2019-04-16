use super::{RGBColor,Material};
use image::{GenericImage, DynamicImage, Pixel};
use crate::graphics::{Point3D, Vector3D,Ray,GeometricObject,World,ViewPlane};


pub struct SimpleTracer;

impl SimpleTracer {
    pub fn new() -> Self {
        SimpleTracer {}
    }

    pub fn render(&self, world: &World) -> DynamicImage {
        let z_plane = -10.0;
        let mut img =
            DynamicImage::new_rgb8(world.get_view_plane().get_hres(), world.get_view_plane().get_vres());

        let vres=world.get_view_plane().get_vres();
        let hres=world.get_view_plane().get_hres();
        let pixel_size=world.get_view_plane().get_pixel_size();
        for y in -((vres/ 2) as i32)..(vres/ 2) as i32 {
            for x in -((hres/ 2) as i32)..(hres/ 2) as i32 {
                let in_world_x = x as f64 * pixel_size - pixel_size / 2.0;
                let in_world_y = y as f64 * pixel_size - pixel_size / 2.0;

                let ray_direction=Point3D::new(in_world_x, in_world_y, z_plane)-
                                                    Point3D::new(0.0,0.0,0.0);
                let ray = Ray::new(Point3D::new(in_world_x, in_world_y, z_plane),
                                   ray_direction.normalize());

                let img_x = (x + (hres/ 2) as i32) as u32;
                let img_y = ((vres/ 2) as i32-y-1) as u32;

                let pixel_color=self.trace_ray(&ray,&world);
                img.put_pixel(img_x, img_y, image::Rgba::from_channels(
                    pixel_color.r_in_8_bit(),pixel_color.g_in_8_bit(),pixel_color.b_in_8_bit(), 255));
            }
        }
        img
    }

    fn trace_ray(&self,ray:&Ray,world:&World)->RGBColor{
        let mut min=std::f64::MAX;
        let mut color=world.get_bg_color().clone();
        for object in world.get_objects(){
            match object.check_hit(ray){
                Some(hitinfo)=>{
                    if hitinfo.get_tmin()<min{
                        min=hitinfo.get_tmin();
                        //color=object.get_material().get_color();
                        let normal=hitinfo.get_normal().normalize();
                        color=RGBColor::new((normal.x()+1.0)/2.0,(normal.y()+1.0)/2.0,(normal.z()+1.0)/2.0);
                    }
                },
                None=>{}
            }
        }
        color
    }
}