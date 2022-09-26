use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::rtweekend::{clamp, random_double};
use crate::vec3::{Color, Point3, Vec3};
use std::io;
use std::io::Write;
use std::rc::Rc;

mod aabb;
mod camera;
mod hittable;
mod material;
mod moving_sphere;
mod ray;
mod rtweekend;
mod vec3;
mod bvh;

fn main() {
    // Image
    let aspect_ratio: f32 = 3.0 / 2.0;
    let img_width = 400; //1200; //200;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100; //500;
    let max_depth = 50;
    // World
    let world = random_scene();
    // Camera
    let look_from = Point3::new(Some(13.0), Some(2.0), Some(3.0));
    let look_at = Point3::new(None, None, None);
    let v_up = Point3::new(None, Some(1.0), None);
    let dist_to_focus = 10.0;
    let aperture: f32 = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
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
                let u = (i as f32 + random_double(None, None)) / (img_width - 1) as f32;
                let v = (j as f32 + random_double(None, None)) / (img_height - 1) as f32;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, max_depth);
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

fn ray_color(ray: Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut hit_rec = HitRecord::empty();

    if depth <= 0 {
        return Color::new(None, None, None);
    }

    if world.hit(&ray, 0.001, crate::rtweekend::INFINITY, &mut hit_rec) {
        let mut scattered = Ray::new(None, None, None);
        let mut attenuation = Color::new(None, None, None);

        if hit_rec.material_ptr.as_ref().unwrap().scatter(
            &ray,
            &hit_rec,
            &mut attenuation,
            &mut scattered,
        ) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }

        return Color::new(None, None, None);
    }

    let unit_direction = Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color::new(Some(1.0), Some(1.0), Some(1.0)) * (1.0 - t)
        + Color::new(Some(0.5), Some(0.7), Some(1.0)) * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new(None);
    let mat_ground = Rc::new(Lambertian::new(Color::new(Some(0.5), Some(0.5), Some(0.5))));
    world.add(Rc::new(Sphere::new(
        Point3::new(None, Some(-1000.0), None),
        1000.0,
        mat_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(None, None);
            let center = Point3::new(
                Some(a as f32 + 0.9 * random_double(None, None)),
                Some(0.2),
                Some(b as f32 + 0.9 * random_double(None, None)),
            );

            if (center - Point3::new(Some(4.0), Some(0.2), None)).len() > 0.9 {
                let mat_sphere: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random(None, None) * Color::random(None, None);
                    mat_sphere = Rc::new(Lambertian::new(albedo));
                    let center2 =
                        center + Vec3::new(None, Some(random_double(None, Some(0.5))), None);
                    world.add(Rc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, mat_sphere,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(Some(0.5), Some(1.0));
                    let fuzz = random_double(None, Some(0.5));
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
    let mat2 = Rc::new(Lambertian::new(Color::new(Some(0.4), Some(0.2), Some(0.1))));
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
