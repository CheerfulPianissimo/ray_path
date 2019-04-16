use super::{RGBColor,Material};
use image::{GenericImage, DynamicImage, Pixel};
use crate::graphics::{Ray, HasMaterial};
use crate::graphics::{Point3D, Vector3D,Hittable,World,ViewPlane};
use crate::shapes::Sphere;


pub struct SimpleTracer;

impl SimpleTracer {
    pub fn new() -> Self {
        SimpleTracer {}
    }

    pub fn render(&self, world: &World) -> DynamicImage {
        let z_plane = 100.0;
        let mut img =
            DynamicImage::new_rgb8(world.get_view_plane().get_hres(), world.get_view_plane().get_vres());

        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 100.0, Material::new(RGBColor::new(1.0, 0.0, 0.0)));

        for y in -((world.get_view_plane().get_vres() / 2) as i32)..(world.get_view_plane().get_vres() / 2) as i32 {
            for x in -((world.get_view_plane().get_hres() / 2) as i32)..(world.get_view_plane().get_hres() / 2) as i32 {
                let in_world_x = x as f64 * world.get_view_plane().get_pixel_size() - world.get_view_plane().get_pixel_size() / 2.0;
                let in_world_y = y as f64 * world.get_view_plane().get_pixel_size() - world.get_view_plane().get_pixel_size() / 2.0;

                let ray = Ray::new(Point3D::new(in_world_x, in_world_y, z_plane), Vector3D::new(0.0, 0.0, -1.0));

                let img_x = (x + (world.get_view_plane().get_hres() / 2) as i32) as u32;
                let img_y = (y + (world.get_view_plane().get_vres() / 2) as i32) as u32;
                //print!("{} {} ",in_world_x,in_world_y);

                if sphere.check_hit(&ray).is_some() {
                    img.put_pixel(img_x, img_y, image::Rgba::from_channels(sphere.get_material().get_color().r_in_8_bit(),
                                                                           sphere.get_material().get_color().g_in_8_bit(),
                                                                           sphere.get_material().get_color().b_in_8_bit(), 255));
                } else {
                    img.put_pixel(img_x, img_y, image::Rgba::from_channels(world.get_bg_color().r_in_8_bit(),
                                                                           world.get_bg_color().g_in_8_bit(), world.get_bg_color().b_in_8_bit(), 255));
                }
            }
        }
        img
    }
}