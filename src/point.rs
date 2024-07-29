use num_traits::{Float, NumOps};

/// `Point` is a point in space that can be added, subtracted, multiplied, and scaled.
/// This trait can be implemented for any type like Point, Vector, Color, etc.
pub trait Point: Clone + PartialEq {
    /// The precision of the point.
    /// It can be f32, f64, or any other type that implements Float.
    type Scalar: Float + NumOps + Copy;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn scale(&self, s: Self::Scalar) -> Self;
}

impl Point for f32 {
    type Scalar = f32;

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn multiply(&self, other: &f32) -> Self {
        self * other
    }

    fn scale(&self, s: f32) -> Self {
        self * s
    }
}

impl Point for f64 {
    type Scalar = f64;

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn multiply(&self, other: &f64) -> Self {
        self * other
    }

    fn scale(&self, s: f64) -> Self {
        self * s
    }
}
