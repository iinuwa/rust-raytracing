mod tests;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);
pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}
impl Color for Vec3 {
    fn r(&self) -> u8 {
        self.0 as u8
    }

    fn g(&self) -> u8 {
        self.1 as u8
    }

    fn b(&self) -> u8 {
        self.2 as u8
    }
}

pub trait Coordinate<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

pub trait Vector<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;

    fn length(&self) -> T;
    fn unit_vector(&self) -> Self;
    fn dot(v1: &Vec3, v2: &Vec3) -> f32;
    fn squared_length(&self) -> f32;
    fn cross(v1: &Vec3, v2: &Vec3) -> Vec3;
}

impl Vector<f32> for Vec3 {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }

    fn z(&self) -> f32 {
        self.2
    }

    fn length(&self) -> f32 {
        ((self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)).sqrt()
    }

    fn unit_vector(&self) -> Self {
        let length = self.length();
        *self / length
    }

    fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        let v = v1 * v2;
        v.0 + v.1 + v.2
    }

    fn squared_length(&self) -> f32 {
        Self::dot(&self, &self)
    }

    fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3(
            v1.1 * v2.2 - v1.2 * v2.1,
            -(v1.0 * v2.2 - v1.2 * v2.0),
            v1.0 * v2.1 - v1.1 * v2.0,
        )
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}
impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

/*
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
*/

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &Vec3) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
