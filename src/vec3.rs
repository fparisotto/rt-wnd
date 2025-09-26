use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::cell::RefCell;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    e: [f32; 3],
}

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(42));
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };

    pub const fn empty() -> Vec3 {
        Self::ZERO
    }

    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    #[inline]
    #[must_use]
    pub fn x(self) -> f32 {
        self.e[0]
    }

    #[inline]
    #[must_use]
    pub fn y(self) -> f32 {
        self.e[1]
    }

    #[inline]
    #[must_use]
    pub fn z(self) -> f32 {
        self.e[2]
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    #[must_use]
    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    #[inline]
    #[must_use]
    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    #[inline]
    #[must_use]
    pub fn unit(self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn fast_unit(self) -> Vec3 {
        let len_squared = self.length_squared();
        if len_squared > 0.0 {
            let inv_len = 1.0 / len_squared.sqrt();
            self * inv_len
        } else {
            self
        }
    }

    pub fn random() -> Vec3 {
        RNG.with(|rng| {
            let mut rng = rng.borrow_mut();
            Vec3::new(rng.random(), rng.random(), rng.random())
        })
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        RNG.with(|rng| {
            let mut rng = rng.borrow_mut();
            Vec3::new(
                rng.random_range(min..max),
                rng.random_range(min..max),
                rng.random_range(min..max),
            )
        })
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    #[inline]
    #[must_use]
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    #[inline]
    #[must_use]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    #[inline]
    #[must_use]
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n).min(1.0);
        let r_out_perp: Vec3 = etai_over_etat * (uv + (cos_theta * n));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        RNG.with(|rng| {
            let mut rng = rng.borrow_mut();
            loop {
                let p = Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                    0.0,
                );
                if p.length_squared() >= 1.0 {
                    continue;
                }
                return p;
            }
        })
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self::Output {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self::Output {
        self * (1.0 / other)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self *= 1.0 / other;
    }
}
