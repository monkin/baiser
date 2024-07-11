use num_traits::Float;
use crate::{Curve, CurvePoint, Distance};
use crate::smooth_array::SmoothArray;

/// The same curve as a passed one, but with a linear dependency between the time and the distance.
pub struct LinearSpeed<F: Float, P: CurvePoint<F> + Distance<F>, C: Curve<F, P>> {
    curve: C,
    length: F,
    table: SmoothArray<F>,
}

impl<F: Float, P: CurvePoint<F>, C: Curve<F, P>> LinearSpeed<F, P, C> {
    pub fn new(curve: C, table_size: usize, steps_count: usize) -> Self {
        let mut table = SmoothArray::with_steps_count(table_size);

        let mut last_point = curve.value_at(F::zero());
        let mut total_length = F::zero();

        let mut t_by_offset: Vec<(F, F)> = Vec::with_capacity(steps_count + 1);
        t_by_offset.push((F::zero(), F::zero()));

        let inverted_steps = F::one() / F::from(steps_count).unwrap();

        for i in 1..=steps_count {
            let t = F::from(i).unwrap() * inverted_steps;
            let point = curve.value_at(t);
            let segment_length = last_point.distance_to(&point);
            total_length = total_length + segment_length;
            t_by_offset.push((total_length, t));
            last_point = point;
        }

        let inverted_length = F::one() / total_length;
        t_by_offset.windows(2).for_each(|window| {
            let (offset1, t1) = window[0];
            let (offset2, t2) = window[1];
            table.line((offset1 * inverted_length, t1), (offset2 * inverted_length, t2));
        });

        Self {
            curve,
            length,
            table,
        }
    }
}

impl<F: Float, P: CurvePoint<F>, C: Curve<F, P>> Curve<F, P> for LinearSpeed<F, P, C> {
    fn value_at(&self, t: F) -> P {
        self.curve.value_at(t)
    }

    fn tangent_at(&self, t: F) -> P {
        let t = t.clamp(F::zero(), F::one());
        let t = self.table.value_at(t);
        self.curve.tangent_at(t).multiply(&self.table.tangent_at(t))
    }

    fn start_point(&self) -> P {
        self.curve.start_point()
    }

    fn end_point(&self) -> P {
        self.curve.end_point()
    }

    fn estimate_length(&self, _precision: F) -> F {
        self.length
    }
}