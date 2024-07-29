use crate::{Curve, Point};
use num_traits::{NumCast, One, Zero};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, PartialEq)]
pub struct CurveIterator<P: Point, C: Curve<P>> {
    curve: C,
    steps_count: P::Scalar,
    include_last: bool,
    i: P::Scalar,
    phantom_data: PhantomData<P>,
}

impl<P: Point + Debug, C: Curve<P> + Debug> Debug for CurveIterator<P, C>
where
    P::Scalar: Debug,
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
impl<P: Point + Copy, C: Curve<P>> Copy for CurveIterator<P, C> where C: Copy {}

impl<P: Point, C: Curve<P>> CurveIterator<P, C> {
    pub fn new(curve: C, steps_count: usize, include_last: bool) -> Self {
        Self {
            curve,
            steps_count: NumCast::from(steps_count).unwrap(),
            include_last,
            i: P::Scalar::zero(),
            phantom_data: Default::default(),
        }
    }
}

impl<P: Point, C: Curve<P>> Iterator for CurveIterator<P, C> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.steps_count || (self.include_last && self.i == self.steps_count) {
            let i = self.i / self.steps_count;
            self.i = self.i + P::Scalar::one();

            Some(self.curve.value_at(i))
        } else {
            None
        }
    }
}
