use crate::smooth_array::SmoothArray;
use crate::{Curve, Distance, Point};
use num_traits::{Float, NumCast, One, Zero};
use std::marker::PhantomData;

/// The same curve as a passed one, but with a linear dependency between the time and the distance.
pub struct LinearSpeed<P: Point + Distance, C: Curve<P>> {
    curve: C,
    length: P::Scalar,
    table: SmoothArray<P::Scalar>,
    phantom_data: PhantomData<P>,
}

impl<P: Point + Distance, C: Curve<P>> LinearSpeed<P, C> {
    pub fn new(curve: C, table_size: usize, steps_count: usize) -> Self {
        let mut table = SmoothArray::with_steps_count(table_size);

        let mut last_point = curve.value_at(P::Scalar::zero());
        let mut total_length = P::Scalar::zero();

        let mut t_by_offset: Vec<(P::Scalar, P::Scalar)> = Vec::with_capacity(steps_count + 1);
        t_by_offset.push((P::Scalar::zero(), P::Scalar::zero()));

        let inverted_steps: P::Scalar = P::Scalar::one() / NumCast::from(steps_count).unwrap();

        for i in 1..=steps_count {
            let i: P::Scalar = NumCast::from(i).unwrap();
            let t: P::Scalar = i * inverted_steps;
            let point = curve.value_at(t);
            let segment_length = last_point.distance(&point);
            total_length = total_length + segment_length;
            t_by_offset.push((total_length, t));
            last_point = point;
        }

        let inverted_length: P::Scalar = P::Scalar::one() / total_length;
        t_by_offset.windows(2).for_each(|window| {
            let (offset1, t1) = window[0];
            let (offset2, t2) = window[1];
            table.line(
                (offset1 * inverted_length, t1),
                (offset2 * inverted_length, t2),
            );
        });

        Self {
            curve,
            length: total_length,
            table,
            phantom_data: Default::default(),
        }
    }
}

impl<P: Point + Distance, C: Curve<P>> Curve<P> for LinearSpeed<P, C> {
    fn value_at(&self, t: P::Scalar) -> P {
        self.curve.value_at(t)
    }

    fn tangent_at(&self, t: P::Scalar) -> P {
        let t = t.clamp(P::Scalar::zero(), P::Scalar::one());
        let t = self.table.value_at(t);
        self.curve.tangent_at(t).scale(self.table.tangent_at(t))
    }

    fn start_point(&self) -> P {
        self.curve.start_point()
    }

    fn end_point(&self) -> P {
        self.curve.end_point()
    }

    fn estimate_length(&self, _precision: P::Scalar) -> P::Scalar {
        self.length
    }
}
