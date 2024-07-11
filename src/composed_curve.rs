use crate::bezier::Bezier;
use crate::{Bezier1, Bezier2, Bezier3, Curve, CurvePoint, Distance};
use num_traits::Float;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Clone, PartialEq)]
pub struct ComposedCurve<F: Float, P: CurvePoint<F>> {
    last_point: P,
    curves: Vec<Bezier<F, P>>,
}

impl<F: Float, P: CurvePoint<F>> Deref for ComposedCurve<F, P>
where
    P: Copy,
{
    type Target = Vec<Bezier<F, P>>;

    fn deref(&self) -> &Self::Target {
        &self.curves
    }
}

impl<F: Float, P: CurvePoint<F>> Debug for ComposedCurve<F, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComposedCurve")
            .field("last_point", &self.last_point)
            .field("curves", &self.curves)
            .finish()
    }
}

impl<F: Float, P: CurvePoint<F>> ComposedCurve<F, P> {
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

impl<F: Float, P: CurvePoint<F>> Curve<F, P> for ComposedCurve<F, P> {
    fn value_at(&self, t: F) -> P {
        let t = t.clamp(F::zero(), F::one());
        let t = t * F::from(self.curves.len()).unwrap();
        let i = t.floor().to_usize().unwrap();
        let t = t.fract();

        if i == self.curves.len() {
            self.curves[i - 1].end_point()
        } else {
            self.curves[i].value_at(t)
        }
    }

    fn tangent_at(&self, t: F) -> P {
        let len = F::from(self.curves.len()).unwrap();

        let t = t.clamp(F::zero(), F::one());
        let t = t * len;
        let i = t.floor().to_usize().unwrap();
        let t = t.fract();

        if i == self.curves.len() {
            self.curves[i - 1].tangent_at(F::one()).scale(len)
        } else {
            self.curves[i].tangent_at(t).scale(len)
        }
    }

    fn estimate_length(&self, precision: F) -> F
    where
        P: Distance<F>,
    {
        self.curves.iter().fold(F::zero(), |acc, curve| {
            acc + curve.estimate_length(precision)
        })
    }
}
