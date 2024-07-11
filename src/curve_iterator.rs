use crate::{Curve, CurvePoint};
use num_traits::Float;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, PartialEq)]
pub struct CurveIterator<F: Float, P: CurvePoint<F>, C: Curve<F, P>> {
    curve: C,
    steps_count: F,
    include_last: bool,
    i: F,
    phantom_data: PhantomData<P>,
}

impl<F: Float, P: CurvePoint<F>, C: Curve<F, P> + Debug> Debug for CurveIterator<F, P, C>
where
    F: Debug,
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CurveIterator")
            .field("curve", &self.curve)
            .field("steps_count", &self.steps_count)
            .field("include_last", &self.include_last)
            .field("i", &self.i)
            .finish()
    }
}
impl<F: Float, P: CurvePoint<F>, C: Curve<F, P>> Copy for CurveIterator<F, P, C> where C: Copy {}

impl<F: Float, P: CurvePoint<F>, C: Curve<F, P>> CurveIterator<F, P, C> {
    pub fn new(curve: C, steps_count: usize, include_last: bool) -> Self {
        Self {
            curve,
            steps_count: F::from(steps_count).unwrap(),
            include_last,
            i: F::zero(),
            phantom_data: Default::default(),
        }
    }
}

impl<F: Float, P: CurvePoint<F>, C: Curve<F, P>> Iterator for CurveIterator<F, P, C> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.steps_count || (self.include_last && self.i == self.steps_count) {
            let i = self.i / self.steps_count;
            self.i = self.i + F::one();

            Some(self.curve.value_at(i))
        } else {
            None
        }
    }
}
