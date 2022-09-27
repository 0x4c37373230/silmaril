use crate::texture::Texture;
use crate::{random, Color, HitRecord, Ray, Vec3};
use std::rc::Rc;
use crate::texture::SolidColor;

pub trait Material {
    fn scatter(
        &self,
        ray_input: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn from(albedo: &Color) -> Lambertian {
        Lambertian {
            albedo: Rc::new(SolidColor::new(Some(*albedo))),
        }
    }
    pub fn new(albedo: Rc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_input: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }

        *scattered = Ray::new(
            Some(hit_rec.point),
            Some(scatter_direction),
            Some(ray_input.time()),
        );
        *attenuation = self.albedo.value(hit_rec.u, hit_rec.v, &hit_rec.point);

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_input: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(ray_input.direction()), &hit_rec.normal);
        *scattered = Ray::new(
            Some(hit_rec.point),
            Some(reflected + Vec3::random_in_unit_sphere() * self.fuzz),
            Some(ray_input.time()),
        );
        *attenuation = self.albedo;

        scattered.direction().dot(&hit_rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(r_in: f32) -> Dielectric {
        Dielectric {
            refraction_index: r_in,
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_input: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(Some(1.0), Some(1.0), Some(1.0));
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = Vec3::unit_vector(ray_input.direction());
        let cos_theta = f32::min((-unit_direction).dot(&hit_rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f32>(0.0, 1.0)
        {
            direction = Vec3::reflect(&unit_direction, &hit_rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &hit_rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(Some(hit_rec.point), Some(direction), Some(ray_input.time()));

        true
    }
}
