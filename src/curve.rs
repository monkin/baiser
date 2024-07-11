use crate::bezier::{Bezier0, Bezier1, Bezier2, Bezier3};
use crate::curve_iterator::CurveIterator;
use crate::curve_point::CurvePoint;
use crate::Distance;
use num_traits::Float;
use crate::composed_curve::ComposedCurve;
use crate::linear_speed::LinearSpeed;

/// A curve is a parametric function that maps a value `t` in range from 0 to 1 to a point in space.
pub trait Curve<F: Float, P: CurvePoint<F>> {
    /// Get the point at a given value `t` in range from 0 to 1.
    fn value_at(&self, t: F) -> P;
    /// Get the derivative at a given value `t` in range from 0 to 1.
    fn tangent_at(&self, t: F) -> P;

    fn start_point(&self) -> P {
        self.value_at(F::zero())
    }

    fn end_point(&self) -> P {
        self.value_at(F::one())
    }

    /// Estimate the length of the curve as an average between `min` and `max` estimation.
    /// The precision parameter is the maximum ration of `min` and `max` estimation.
    ///
    /// Precision:
    ///   * **F::infinity()** - means that estimation will be done in one step,
    ///   * **1.0** - means that `max / min` should be less than `100%`,
    ///   * **0.5** - the same as above, but the difference is `50%`,
    ///   * **0.1** - the same as above, but the difference is `10%`,
    ///   * and so on...
    fn estimate_length(&self, precision: F) -> F
    where
        P: Distance<F>;

    /// Create a dot, at any `t` it will return the same value
    fn dot(p0: P) -> Bezier0<F, P> {
        Bezier0(p0)
    }

    /// Create a line
    fn line(p0: P, p1: P) -> Bezier1<F, P> {
        Bezier1(p0, p1)
    }

    /// Create a quadratic bezier curve
    fn quad_bezier(p0: P, p1: P, p2: P) -> Bezier2<F, P> {
        Bezier2(p0, p1, p2)
    }


    /// Create a cubic bezier curve
    fn cubic_bezier(p0: P, p1: P, p2: P, p3: P) -> Bezier3<F, P> {
        Bezier3(p0, p1, p2, p3)
    }

    /// Create an iterator that will generate points on the curve.
    fn into_iter(self, steps_count: usize) -> CurveIterator<F, P, Self> {
        CurveIterator::new(self, steps_count, false)
    }

    /// Create an iterator that will generate points on the curve, including the last point.
    fn into_iter_inclusive(self, steps_count: usize) -> CurveIterator<F, P, Self> {
        CurveIterator::new(self, steps_count, true)
    }

    /// Create a composed curve that will be a sequence of curves.
    /// Each segment of the curve will be represented by equal `t` range.
    /// For example, if you have 3 curves, they will take `t` ranges: `0 - 0.33`, `0.33 - 0.66` and `0.66 - 1.0`.
    fn composed_curve(start_point: P) -> ComposedCurve<F, P> {
        ComposedCurve::new(start_point)
    }

    /// Create a linear speed curve that will allow to move with a constant speed along the curve.
    /// It's especially useful when you want to animate the movement along a composed curve.
    ///
    /// Arguments:
    /// * `table_size` - the size of the table that will be used to speed up the calculations,
    ///     the bigger means the better the precision.
    /// * `steps_count` - the number of steps that will be used to calculate the table,
    ///     so if you have 3 steps then the curve points will be calculated at 0.0, 0.5 and 1.0.
    ///     Intermediate points will be interpolated.
    fn linear_speed(self, table_size: usize, steps_count: usize) -> LinearSpeed<F, P, Self> where P: Distance<F>{
        LinearSpeed::new(self, table_size, steps_count)
    }
}
