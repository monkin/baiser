use num_traits::Float;

/// CurvePoint is a point in space that can be added, subtracted, multiplied, and scaled.
/// This trait can be implemented for any type like Point, Vector, Color, etc.
pub trait CurvePoint<F: Float>: Clone + PartialEq {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn scale(&self, s: F) -> Self;
}

impl<F: Float> CurvePoint<F> for f32 {
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn multiply(&self, other: &f32) -> Self {
        self * other
    }

    fn scale(&self, s: F) -> Self {
        self * s.to_f32().unwrap()
    }
}

impl<F: Float> CurvePoint<F> for f64 {
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn multiply(&self, other: &f64) -> Self {
        self * other
    }

    fn scale(&self, s: F) -> Self {
        self * s.to_f64().unwrap()
    }
}
