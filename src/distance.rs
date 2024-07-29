use crate::Point;

pub trait Distance: Point {
    fn distance(&self, other: &Self) -> Self::Scalar;
}

impl Distance for f32 {
    fn distance(&self, other: &Self) -> Self::Scalar {
        (self - other).abs()
    }
}

impl Distance for f64 {
    fn distance(&self, other: &Self) -> Self::Scalar {
        (self - other).abs()
    }
}
