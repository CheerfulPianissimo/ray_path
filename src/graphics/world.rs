use super::{RGBColor,Material};
use image::{GenericImage, DynamicImage, ImageBuffer, Pixel};
use crate::graphics::{Ray, HasMaterial};
use crate::graphics::{Point3D, Vector3D,Hittable};
use crate::shapes::Sphere;

pub struct ViewPlane{
    hres:u32,
    vres:u32,
    ///Pixel size: Number of  in-world units corresponding to a pixel
    s:f64,
}

impl ViewPlane {
    pub fn new(hres: u32, vres: u32, s: f64) -> Self {
        ViewPlane { hres, vres, s }
    }
}

pub struct World{
    v_plane:ViewPlane,
    bg_color:RGBColor,
}

impl World {
    pub fn new(v_plane: ViewPlane, bg_color: RGBColor) -> Self {
        World {v_plane , bg_color }
    }

    pub fn render(&self){
        let in_world_width=self.v_plane.hres as f64*self.v_plane.s;
        let in_world_height=self.v_plane.vres as f64*self.v_plane.s;
        let z_plane=100.0;
        let mut img =
            DynamicImage::new_rgb8(self.v_plane.hres, self.v_plane.vres);

        let sphere=Sphere::new(Point3D::new(0.0,0.0,0.0),100.0,Material::new(RGBColor::new(1.0,0.0,0.0)));

        for y in -((self.v_plane.vres/2) as i32)..(self.v_plane.vres/2) as i32{
            for x in -((self.v_plane.hres/2) as i32)..(self.v_plane.hres/2) as i32{
                let in_world_x=x as f64*self.v_plane.s-self.v_plane.s/2.0;
                let in_world_y=y as f64*self.v_plane.s-self.v_plane.s/2.0;

                let ray=Ray::new(Point3D::new(in_world_x,in_world_y,z_plane),Vector3D::new(0.0,0.0,-1.0));

                let img_x=(x+(self.v_plane.hres/2) as i32) as u32;
                let img_y=(y+(self.v_plane.vres/2) as i32) as u32;
                //print!("{} {} ",in_world_x,in_world_y);

                if sphere.check_hit(&ray).is_some(){

                    img.put_pixel(img_x ,img_y , image::Rgba::from_channels(sphere.get_material().get_color().r_in_8_bit(),
                                                                   sphere.get_material().get_color().g_in_8_bit(),
                                                                 sphere.get_material().get_color().b_in_8_bit(),255));
                }else{
                    img.put_pixel(img_x,img_y, image::Rgba::from_channels(self.bg_color.r_in_8_bit(),
                                                                                         self.bg_color.g_in_8_bit(),self.bg_color.b_in_8_bit(),255));
                }   //                 img.put_pixel(img_x, img_y, image::Rgba::from_channels(250,250,250,250));

            }
        }

        img.save("./img.jpeg");
    }
}

