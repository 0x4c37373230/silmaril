use crate::aarect::{XYRect, XZRect, YZRect};
use crate::block::Block;
use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::rtweekend::{clamp, random, INFINITY};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Color, Point3, Vec3};
use std::io;
use std::io::Write;
use std::rc::Rc;

mod aabb;
mod aarect;
mod block;
mod bvh;
mod camera;
mod hittable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod texture;
mod vec3;

fn main() {
    // Image
    let mut aspect_ratio: f32 = 3.0 / 2.0;
    let mut img_width = 400; //1200; //200;
    let mut samples_per_pixel = 100; //500;
    let max_depth = 50;
    // World
    let world: HittableList;
    let mut look_from = Point3::new(Some(13.0), Some(2.0), Some(3.0));
    let mut look_at = Point3::new(None, None, None);
    let mut v_fov: f32 = 20.0; //40.0;
    let mut aperture: f32 = 0.0;
    let mut background = Color::new(Some(0.70), Some(0.80), Some(1.0));

    match 0 {
        1 => {
            world = random_scene();
            aperture = 0.1;
        }
        2 => world = two_spheres(),
        3 => world = two_perlin_spheres(),
        4 => world = earth(),
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::new(None, None, None);
            look_from = Point3::new(Some(26.0), Some(3.0), Some(6.0));
            look_at = Point3::new(None, Some(2.0), None);
        }
        6 | _ => {
            world = cornell_box();
            aspect_ratio = 1.0;
            img_width = 600;
            samples_per_pixel = 200;
            background = Color::new(None, None, None);
            look_from = Point3::new(Some(278.0), Some(278.0), Some(-800.0));
            look_at = Point3::new(Some(278.0), Some(278.0), None);
            v_fov = 40.0;
        }
    }
    // Image 2
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    // Camera
    let v_up = Point3::new(None, Some(1.0), None);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        look_from,
        look_at,
        v_up,
        v_fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        None,
        Some(1.0),
    );
    // Render
    // Output will be in the PPM format, this sets up the metadata. P3 means the colors are in
    // ASCII format, img width and height represent columns and rows and 255 is the max value
    println!("P3\n{} {}\n255", img_width, img_height);

    for j in (0..img_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();

        for i in 0..img_width {
            let mut pixel_color = Color::new(None, None, None);

            for _s in 0..samples_per_pixel {
                let u = (i as f32 + random::<f32>(0.0, 1.0)) / (img_width - 1) as f32;
                let v = (j as f32 + random::<f32>(0.0, 1.0)) / (img_height - 1) as f32;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &background, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel) // PPM file contents
        }
    }
}

// This function writes the color of a single pixel to stdout
fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    )
}

fn ray_color(ray: Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    let mut hit_rec = HitRecord::empty();

    if depth <= 0 {
        return Color::new(None, None, None);
    }

    if !world.hit(&ray, 0.001, INFINITY, &mut hit_rec) {
        return *background;
    }

    let mut scattered = Ray::new(None, None, None);
    let mut attenuation = Color::new(None, None, None);
    let emitted =
        hit_rec
            .material_ptr
            .as_ref()
            .unwrap()
            .emitted(hit_rec.u, hit_rec.v, &hit_rec.point);

    if !hit_rec.material_ptr.as_ref().unwrap().scatter(
        &ray,
        &hit_rec,
        &mut attenuation,
        &mut scattered,
    ) {
        return emitted;
    }

    emitted + attenuation * ray_color(scattered, background, world, depth - 1)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new(None);
    /* let mat_ground = Rc::new(Lambertian::from(&Color::new(
        Some(0.5),
        Some(0.5),
        Some(0.5),
    ))); */
    let checker = Rc::new(CheckerTexture::from(
        Color::new(Some(0.2), Some(0.3), Some(0.1)),
        Color::new(Some(0.9), Some(0.9), Some(0.9)),
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new(None, Some(-1000.0), None),
        1000.0,
        Rc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>(0.0, 1.0);
            let center = Point3::new(
                Some(a as f32 + 0.9 * random::<f32>(0.0, 1.0)),
                Some(0.2),
                Some(b as f32 + 0.9 * random::<f32>(0.0, 1.0)),
            );

            if (center - Point3::new(Some(4.0), Some(0.2), None)).len() > 0.9 {
                let mat_sphere: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random(None, None) * Color::random(None, None);
                    mat_sphere = Rc::new(Lambertian::from(&albedo));
                    let center2 = center + Vec3::new(None, Some(random(0.0, 0.5)), None);
                    world.add(Rc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, mat_sphere,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(Some(0.5), Some(1.0));
                    let fuzz = random::<f32>(0.0, 0.5);
                    mat_sphere = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat_sphere)));
                } else {
                    mat_sphere = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat_sphere)));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(None, Some(1.0), None),
        1.0,
        mat1,
    )));
    let mat2 = Rc::new(Lambertian::from(&Color::new(
        Some(0.4),
        Some(0.2),
        Some(0.1),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(Some(-4.0), Some(1.0), None),
        1.0,
        mat2,
    )));
    let mat3 = Rc::new(Metal::new(Color::new(Some(0.7), Some(0.6), Some(0.5)), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(Some(4.0), Some(1.0), None),
        1.0,
        mat3,
    )));

    return world;
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new(None);
    let checker = Rc::new(CheckerTexture::from(
        Color::new(Some(0.2), Some(0.3), Some(0.1)),
        Color::new(Some(0.9), Some(0.9), Some(0.9)),
    ));
    objects.add(Rc::new(Sphere::new(
        Point3::new(None, Some(-10.0), None),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(None, Some(10.0), None),
        10.0,
        Rc::new(Lambertian::new(checker)),
    )));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new(None);
    let perlin_texture = Rc::new(NoiseTexture::new(Some(4.0)));
    objects.add(Rc::new(Sphere::new(
        Point3::new(None, Some(-1000.0), None),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(None, Some(2.0), None),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));

    objects
}

fn earth() -> HittableList {
    let earth_texture = Rc::new(ImageTexture::new("../../textures/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));
    let globe = Rc::new(Sphere::new(Vec3::new(None, None, None), 2.0, earth_surface));

    HittableList::new(Some(globe))
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new(None);
    let perlin_texture = Rc::new(NoiseTexture::new(Some(4.0)));
    objects.add(Rc::new(Sphere::new(
        Point3::new(None, Some(-1000.0), None),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(None, Some(2.0), None),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));
    let diff_light = Rc::new(DiffuseLight::from(Color::new(
        Some(4.0),
        Some(4.0),
        Some(4.0),
    )));
    objects.add(Rc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Some(diff_light),
    )));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new(None);
    let red = Rc::new(Lambertian::from(&Color::new(
        Some(0.65),
        Some(0.05),
        Some(0.05),
    )));
    let white = Rc::new(Lambertian::from(&Color::new(
        Some(0.73),
        Some(0.73),
        Some(0.73),
    )));
    let green = Rc::new(Lambertian::from(&Color::new(
        Some(0.12),
        Some(0.45),
        Some(0.15),
    )));
    let light = Rc::new(DiffuseLight::from(Color::new(
        Some(15.0),
        Some(15.0),
        Some(15.0),
    )));

    objects.add(Rc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Some(green),
    )));
    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Some(red))));
    objects.add(Rc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Some(light),
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Some(white.clone()),
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Some(white.clone()),
    )));
    objects.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Some(white.clone()),
    )));
    objects.add(Rc::new(Block::new(
        &Point3::new(Some(130.0), None, Some(65.0)),
        &Point3::new(Some(295.0), Some(165.0), Some(230.0)),
        white.clone(),
    )));
    objects.add(Rc::new(Block::new(
        &Point3::new(Some(265.0), None, Some(295.0)),
        &Point3::new(Some(430.0), Some(330.0), Some(460.0)),
        white,
    )));

    objects
}
