use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub(crate) e: [f32; 3],
}

impl Vec3 {
    // This acts as a constructor. The parameters being Option<T> are a hacky way of implementing
    // default parameters with unwrap_or(). Pass 'None' if you want default values, or Some(Value)
    // if you actually want to provide something. This same method(?) is used across most
    // constructors in this program
    pub fn new(x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Vec3 {
        Vec3 {
            e: [x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0)],
        }
    }

    // Getters
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn len_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    // A Vec3 calculates the dot product of itself and another vector.  The 'v' parameter simply
    // specifies the 2nd vector
    pub fn dot(&self, v: &Vec3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    // A Vec3 calculates the cross product of itself and another vector. The 'v' parameter
    // simply specifies the 2nd vector
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            Some(self.e[1] * v.e[2] - self.e[2] * v.e[1]),
            Some(self.e[2] * v.e[0] - self.e[0] * v.e[2]),
            Some(self.e[0] * v.e[1] - self.e[1] * v.e[0]),
        )
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        let length = v.len();
        v / length
    }

    // Basically another type of constructor only that this time, it generates a Vec3 with random
    // X, Y and Z values within the specified range on the parameters, or the defaults. See the
    // random_double() function
    pub fn random(min: Option<f32>, max: Option<f32>) -> Vec3 {
        Vec3::new(
            Some(random::<f32>(min.unwrap(), max.unwrap())),
            Some(random::<f32>(min.unwrap(), max.unwrap())),
            Some(random::<f32>(min.unwrap(), max.unwrap())),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(Some(-1.0), Some(1.0));

            if p.len_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();

        return if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        };
    }

    pub fn near_zero(&self) -> bool {
        let s: f32 = 1e-8;

        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * v.dot(n) * 2.0
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min((-*uv).dot(n) as f32, 1.0);
        let r_out_perp: Vec3 = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel: Vec3 = *n * -((1.0 - r_out_perp.len_squared()).abs().sqrt());

        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                Some(random::<f32>(-1.0, 1.0)),
                Some(random::<f32>(-1.0, 1.0)),
                None,
            );

            if p.len_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-(self.e[0]), -(self.e[1]), -(self.e[2])],
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] *= 1.0 / rhs;
        self.e[1] *= 1.0 / rhs;
        self.e[2] *= 1.0 / rhs
    }
}

use crate::rtweekend::random;
pub use Vec3 as Point3;
pub use Vec3 as Color;

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            Some(self.e[0] + rhs.e[0]),
            Some(self.e[1] + rhs.e[1]),
            Some(self.e[2] + rhs.e[2]),
        )
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            Some(self.e[0] - rhs.e[0]),
            Some(self.e[1] - rhs.e[1]),
            Some(self.e[2] - rhs.e[2]),
        )
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            Some(self.e[0] * rhs.e[0]),
            Some(self.e[1] * rhs.e[1]),
            Some(self.e[2] * rhs.e[2]),
        )
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(
            Some(self.e[0] * rhs),
            Some(self.e[1] * rhs),
            Some(self.e[2] * rhs),
        )
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
