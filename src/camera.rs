use crate::rtweekend::degrees_to_radians;
use crate::{random_double, Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        _time0: Option<f32>,
        _time1: Option<f32>,
    ) -> Camera {
        let theta = degrees_to_radians(vert_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(v_up.cross(&w));
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let llc = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: llc,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: _time0.unwrap_or(0.0),
            time1: _time1.unwrap_or(0.0),
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            Some(self.origin + offset),
            Some(
                self.lower_left_corner + self.horizontal * s + self.vertical * t
                    - self.origin
                    - offset,
            ),
            Some(random_double(Some(self.time0), Some(self.time1))),
        )
    }
}
