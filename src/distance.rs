use num_traits::Float;

pub trait Distance<F: Float> {
    fn distance(&self, other: &Self) -> F;
}

impl<F: Float> Distance<F> for f32 {
    fn distance(&self, other: &Self) -> F {
        F::from((self - other).abs()).unwrap_or(F::infinity())
    }
}

impl<F: Float> Distance<F> for f64 {
    fn distance(&self, other: &Self) -> F {
        F::from((self - other).abs()).unwrap_or(F::infinity())
    }
}
