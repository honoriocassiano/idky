use std::fmt;
use std::ops;

pub use Vec2 as Point2;

use super::optic::{Reflect, Refract};
use super::vector::Vector;

type Scalar = f32;

#[derive(Copy, Clone)]
pub struct Vec2(pub Scalar, pub Scalar);

impl Vec2 {
    pub const fn x(&self) -> Scalar {
        self.0
    }

    pub const fn y(&self) -> Scalar {
        self.1
    }
}

impl Vector for Vec2 {
    type Scalar = Scalar;

    fn zero() -> Self {
        Self(0.0, 0.0)
    }

    fn length(&self) -> Self::Scalar {
        self.sq_length().sqrt()
    }

    fn sq_length(&self) -> Self::Scalar {
        self.0.mul_add(self.0, self.1 * self.1)
    }

    fn normalize(&mut self) -> &mut Self {
        let norm = self.length();
        *self = Self(self.0 / norm, self.1 / norm);

        self
    }

    fn normalized(&self) -> Self {
        let norm = self.length();

        Self(self.0 / norm, self.1 / norm)
    }

    fn dot(&self, v: Self) -> Self::Scalar {
        self.0.mul_add(v.0, self.1 * v.1)
    }

    fn cross(&self, v: Self) -> Self {
        Self(v.1, -v.0)
    }
}

impl Reflect for Vec2 {
    fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * (normal)
    }
}

impl Refract for Vec2 {
    type Scalar = Scalar;

    fn refract(&self, normal: Self, eta_in_over_eta_out: Self::Scalar) -> Self {
        let cos_theta: Self::Scalar = (-(*self)).dot(normal);

        let vec_out_perp: Self = eta_in_over_eta_out * ((*self) + cos_theta * normal);
        let vec_out_par: Self = -(1.0 - vec_out_perp.sq_length()).abs().sqrt() * normal;

        vec_out_perp + vec_out_par
    }
}

// Addition operators
impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

// Subtraction operators
impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1);
    }
}

// Unary negation
impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

// Multiplication operators
impl ops::Mul<Scalar> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: Scalar) -> Self::Output {
        Self(self.0 * scalar, self.1 * scalar)
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl ops::MulAssign<Scalar> for Vec2 {
    fn mul_assign(&mut self, scalar: Scalar) {
        *self = Self(self.0 * scalar, self.1 * scalar);
    }
}

// Right hand scalar multiplication operator
impl ops::Mul<Vec2> for Scalar {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Self::Output {
        Vec2(vec.0 * self, vec.1 * self)
    }
}

// Division operators
impl ops::Div<Scalar> for Vec2 {
    type Output = Self;

    fn div(self, scalar: Scalar) -> Self::Output {
        Self(self.0 / scalar, self.1 / scalar)
    }
}

impl ops::DivAssign<Scalar> for Vec2 {
    fn div_assign(&mut self, scalar: Scalar) {
        *self = Self(self.0 / scalar, self.1 / scalar);
    }
}

// Indexing operators
impl ops::Index<usize> for Vec2 {
    type Output = Scalar;

    fn index(&self, other: usize) -> &Scalar {
        match other {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index {} is not in Vec2", other),
        }
    }
}

impl ops::IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, other: usize) -> &mut Scalar {
        match other {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index {} is not in Vec2", other),
        }
    }
}

// Format string
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-08;

    fn almost_equal(v1: f32, v2: f32) -> bool {
        (v1 - v2).abs() < EPSILON
    }

    fn almost_equal_vec(v1: Vec2, v2: Vec2) -> bool {
        let diff_x = (v1.0 - v2.0).abs();
        let diff_y = (v1.1 - v2.1).abs();

        (diff_x + diff_y) < EPSILON
    }

    // Tests for getters
    #[test]
    #[allow(clippy::float_cmp)]
    fn must_set_coordinates() {
        let expected_x = 0.597;
        let expected_y = 1.47856;

        let vec = Vec2(expected_x, expected_y);

        assert_eq!(expected_x, vec.x());
        assert_eq!(expected_y, vec.y());
    }

    // Tests for Vector trait

    #[test]
    fn must_have_length_zero() {
        let expected = 0.0;

        assert!(almost_equal(expected, Vec2::zero().length()));
    }

    #[test]
    fn must_have_length_one() {
        let expected = 1.0;
        let vec = Vec2(1.0, 0.0);

        assert!(almost_equal(expected, vec.length()));
    }

    #[test]
    fn must_have_sq_length_zero() {
        let expected = 0.0;

        assert!(almost_equal(expected, Vec2::zero().sq_length()));
    }

    #[test]
    fn must_have_sq_length_one() {
        let expected = 1.0;
        let vec = Vec2(1.0, 0.0);

        assert!(almost_equal(expected, vec.sq_length()));
    }

    #[test]
    fn must_have_normalized_length_one() {
        let expected = 1.0;
        let vec = Vec2(1.0, 5.0);

        assert!(almost_equal(expected, vec.normalized().length()));

        let mut mut_vec = vec;
        mut_vec.normalize();

        assert!(almost_equal(expected, mut_vec.length()));
    }

    #[test]
    fn must_be_equal_sum() {
        let expected = Vec2(1.0, 5.3);

        let vec1 = Vec2(0.0, 2.1);
        let vec2 = Vec2(1.0, 3.2);

        assert!(almost_equal_vec(expected, vec1 + vec2));

        let mut mut_vec1 = vec1;
        mut_vec1 += vec2;

        assert!(almost_equal_vec(expected, mut_vec1));
    }

    #[test]
    fn must_be_equal_neg() {
        let expected = Vec2(0.5, -0.9);

        let vec = Vec2(-0.5, 0.9);

        assert!(almost_equal_vec(expected, -vec));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn must_return_coordinate() {
        let expected_0 = 1.0;
        let expected_1 = 5.48;

        let vec = Vec2(expected_0, expected_1);

        assert_eq!(expected_0, vec[0]);
        assert_eq!(expected_1, vec[1]);
    }

    #[test]
    #[should_panic(expected = "Index 2 is not in Vec2")]
    #[allow(clippy::no_effect)]
    fn must_panic_on_invalid_index() {
        let vec = Vec2::zero();

        vec[2];
    }

    // TODO Add Test to other operations
}
