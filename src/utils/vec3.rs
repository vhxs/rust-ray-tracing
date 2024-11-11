use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let small_value = 1e-8;
        return self.x.abs() < small_value
            && self.y.abs() < small_value
            && self.z.abs() < small_value;
    }

    pub fn unit_vector(vector: &Vec3) -> Vec3 {
        return *vector / vector.norm();
    }

    pub fn random_coordinate(min: f64, max: f64) -> f64 {
        let mut rng = rand::thread_rng();
        return rng.gen::<f64>() * (max - min) + min;
    }

    fn random_vector(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: Self::random_coordinate(min, max),
            y: Self::random_coordinate(min, max),
            z: Self::random_coordinate(min, max),
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_vector(-1., 1.);
            let lensq = p.norm_squared();
            if 1e-160 <= lensq && lensq <= 1. {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, normal) > 0. {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn dot(vector1: &Vec3, vector2: &Vec3) -> f64 {
        return vector1.x * vector2.x + vector1.y * vector2.y + vector1.z * vector2.z;
    }

    pub fn cross(vector1: &Vec3, vector2: &Vec3) -> Vec3 {
        return Vec3::new(
            vector1.y * vector2.z - vector1.z * vector2.y,
            vector1.z * vector2.x - vector1.x * vector2.z,
            vector1.x * vector2.y - vector1.y * vector2.x,
        );
    }

    pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
        return *vector - *normal * Self::dot(vector, normal) * 2.;
    }

    pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&-*uv, normal), 1.0);
        let r_out_perp = (*uv + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -(1. - r_out_perp.norm_squared()).abs().sqrt();

        return r_out_perp + r_out_parallel;
    }
}

// this syntax is used to implement what are called "traits" in Rust
// traits are sort of like interfaces. syntax: impl trait for struct
// can declare new traits. in this case though, implementing Add, etc is how
// we do operator overloading in Rust
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// scalar multiplication
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// scalar division
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// elementwise multiplication
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// elementwise division
impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
