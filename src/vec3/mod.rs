mod tests;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/*
pub trait Vec3<T> {
    a: T,
    b: T,
    c: T,

}
*/
pub struct Vec3<T>(pub T, pub T, pub T);
pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

pub trait Vector {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

pub trait Point {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

impl Color for Vec3<u8> {
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
impl Point for Vec3<f32> {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }

    fn z(&self) -> f32 {
        self.2
    }
}

impl<T> PartialEq for Vec3<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec3<T>) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Vec3<T>) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3<f32> {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl<T: MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl MulAssign<f32> for Vec3<f32> {
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f32> for Vec3<f32> {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl<T: DivAssign> DivAssign for Vec3<T> {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl DivAssign<f32> for Vec3<f32> {
    fn div_assign(&mut self, other: f32) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
