use crate::bezier::Bezier;
use crate::{Bezier1, Bezier2, Bezier3, Curve, Distance, Point};
use num_traits::{Float, NumCast, One, ToPrimitive, Zero};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Clone, PartialEq)]
pub struct ComposedCurve<P: Point> {
    last_point: P,
    curves: Vec<Bezier<P>>,
}

impl<P: Point> Deref for ComposedCurve<P>
where
    P: Copy,
{
    type Target = Vec<Bezier<P>>;

    fn deref(&self) -> &Self::Target {
        &self.curves
    }
}

impl<P: Point + Debug> Debug for ComposedCurve<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComposedCurve")
            .field("last_point", &self.last_point)
            .field("curves", &self.curves)
            .finish()
    }
}

impl<P: Point> ComposedCurve<P> {
    pub fn new(start_point: P) -> Self {
        Self {
            last_point: start_point,
            curves: Vec::new(),
        }
    }

    pub fn with_capacity(start_point: P, capacity: usize) -> Self {
        Self {
            last_point: start_point,
            curves: Vec::with_capacity(capacity),
        }
    }

    pub fn line_to(&mut self, point: P) {
        if point != self.last_point {
            let curve = Bezier::C1(Bezier1::new(self.last_point.clone(), point.clone()));
            self.curves.push(curve);
            self.last_point = point;
        }
    }

    pub fn quadratic_to(&mut self, p1: P, p2: P) {
        if p1 == p2 && p1 == self.last_point {
            return;
        }

        let curve = Bezier::C2(Bezier2::new(self.last_point.clone(), p1, p2.clone()));
        self.curves.push(curve);
        self.last_point = p2;
    }

    pub fn cubic_to(&mut self, p1: P, p2: P, p3: P) {
        if p1 == p2 && p2 == p3 && p1 == self.last_point {
            return;
        }

        let curve = Bezier::C3(Bezier3::new(self.last_point.clone(), p1, p2, p3.clone()));
        self.curves.push(curve);
        self.last_point = p3;
    }

    pub fn close(&mut self) {
        if !self.curves.is_empty() {
            let first_point = self.curves[0].start_point();
            self.line_to(first_point);
        }
    }
}

impl<P: Point> Curve<P> for ComposedCurve<P> {
    fn value_at(&self, t: P::Scalar) -> P {
        let t = t.clamp(P::Scalar::zero(), P::Scalar::one());
        let t: P::Scalar = t * NumCast::from(self.curves.len()).unwrap();
        let i = t.floor().to_usize().unwrap();
        let t = t.fract();

        if i == self.curves.len() {
            self.curves[i - 1].end_point()
        } else {
            self.curves[i].value_at(t)
        }
    }

    fn tangent_at(&self, t: P::Scalar) -> P {
        let len: P::Scalar = NumCast::from(self.curves.len()).unwrap();

        let t = t.clamp(Zero::zero(), One::one());
        let t: P::Scalar = t * len;
        let i = t.floor().to_usize().unwrap();
        let t = t.fract();

        if i == self.curves.len() {
            self.curves[i - 1].tangent_at(One::one()).scale(len)
        } else {
            self.curves[i].tangent_at(t).scale(len)
        }
    }

    fn estimate_length(&self, precision: P::Scalar) -> P::Scalar
    where
        P: Distance,
    {
        self.curves.iter().fold(Zero::zero(), |acc, curve| {
            acc + curve.estimate_length(precision)
        })
    }
}
