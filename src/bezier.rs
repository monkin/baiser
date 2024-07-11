use crate::{Curve, CurvePoint, Distance};
use num_traits::Float;
use std::fmt::Debug;
use crate::linear_speed::LinearSpeed;

/// Single point
#[derive(Clone, PartialEq)]
pub struct Bezier0<F: Float, P: CurvePoint<F>>(pub P);

/// Line
#[derive(Clone, PartialEq)]
pub struct Bezier1<F: Float, P: CurvePoint<F>>(pub P, pub P);

/// Quadratic bezier curve
#[derive(Clone, PartialEq)]
pub struct Bezier2<F: Float, P: CurvePoint<F>>(pub P, pub P, pub P);

/// Cubic bezier curve
#[derive(Clone, PartialEq)]
pub struct Bezier3<F: Float, P: CurvePoint<F>>(pub P, pub P, pub P, pub P);

#[derive(Clone, PartialEq)]
pub enum Bezier<F: Float, P: CurvePoint<F>> {
    C0(Bezier0<F, P>),
    C1(Bezier1<F, P>),
    C2(Bezier2<F, P>),
    C3(Bezier3<F, P>),
}

impl<F: Float, P: CurvePoint<F>> Bezier<F, P> {
    fn content<'a>(&'a self) -> &'a dyn Curve<F, P> {
        match self {
            Bezier::C0(c) => c,
            Bezier::C1(c) => c,
            Bezier::C2(c) => c,
            Bezier::C3(c) => c,
        }
    }
}

impl<F: Float, P: CurvePoint<F>> Copy for Bezier<F, P> where P: Copy {}
impl<F: Float, P: CurvePoint<F>> Copy for Bezier0<F, P> where P: Copy {}
impl<F: Float, P: CurvePoint<F>> Copy for Bezier1<F, P> where P: Copy {}
impl<F: Float, P: CurvePoint<F>> Copy for Bezier2<F, P> where P: Copy {}
impl<F: Float, P: CurvePoint<F>> Copy for Bezier3<F, P> where P: Copy {}

impl<F: Float, P: CurvePoint<F>> Debug for Bezier<F, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier")
            .field(match self {
                Bezier::C0(c) => &c,
                Bezier::C1(c) => &c,
                Bezier::C2(c) => &c,
                Bezier::C3(c) => &c,
            })
            .finish()
    }
}
impl<F: Float, P: CurvePoint<F>> Debug for Bezier0<F, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier0").field(&self.0).finish()
    }
}
impl<F: Float, P: CurvePoint<F>> Debug for Bezier1<F, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier1")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<F: Float, P: CurvePoint<F>> Debug for Bezier2<F, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier2")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl<F: Float, P: CurvePoint<F>> Debug for Bezier3<F, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bezier3")
            .field("0", &self.0)
            .field("1", &self.1)
            .field("2", &self.2)
            .field("3", &self.3)
            .finish()
    }
}

impl<F: Float, P: CurvePoint<F>> Curve<F, P> for Bezier0<F, P> {
    fn value_at(&self, _t: F) -> P {
        self.0.clone()
    }

    fn tangent_at(&self, _t: F) -> P {
        P::zero()
    }

    fn start_point(&self) -> P {
        self.0.clone()
    }

    fn end_point(&self) -> P {
        self.0.clone()
    }

    fn estimate_length(&self, _precision: F) -> F
    where
        P: Distance<F>,
    {
        F::zero()
    }
}

impl<F: Float, P: CurvePoint<F>> Curve<F, P> for Bezier1<F, P> {
    fn value_at(&self, t: F) -> P {
        self.0.add(&self.1.sub(&self.0).scale(&t))
    }

    fn tangent_at(&self, _t: F) -> P {
        self.1.sub(&self.0)
    }

    fn start_point(&self) -> P {
        self.0.clone()
    }

    fn end_point(&self) -> P {
        self.1.clone()
    }

    fn estimate_length(&self, _precision: F) -> F
    where
        P: Distance<F>,
    {
        self.0.distance(&self.1)
    }
}

impl<F: Float, P: CurvePoint<F>> Curve<F, P> for Bezier2<F, P> {
    fn value_at(&self, t: F) -> P {
        let t2 = t * t;
        let t1 = F::one() - t;
        let t12 = t1 * t1;

        self.0
            .scale(&t12)
            .add(&self.1.scale(&(F::from_f64(2.0).unwrap() * t1 * t)))
            .add(&self.2.scale(&t2))
    }

    fn tangent_at(&self, t: F) -> P {
        let Bezier2(p1, p2, p3) = self;
        let two = F::one() + F::one();

        let t2 = t + t;
        let nt2 = two - t2;

        let v1 = p2.sub(p1).scale(nt2);
        let v2 = p3.sub(p2).scale(t2);

        v1.add(&v2)
    }

    fn start_point(&self) -> P {
        self.0.clone()
    }

    fn end_point(&self) -> P {
        self.2.clone()
    }

    fn estimate_length(&self, precision: F) -> F
    where
        P: Distance<F>,
    {
        let Bezier2(p1, p2, p3) = self;

        let min = p1.distance(p3);
        let max = p1.distance(p2) + p2.distance(p3);

        let half = F::one() / (F::one() + F::one());

        if max == F::zero() {
            F::zero()
        } else if (max - min) / max < precision {
            (min + max) * half;
        } else {
            let m01 = p1.add(&p2).scale(half);
            let m12 = p2.add(&p3).scale(half);
            let m = m01.add(&m12).scale(half);

            let b1 = Bezier2(p1, m01, m.clone());
            let b2 = Bezier2(m, m12, p3);

            b1.estimate_length(precision) + b2.estimate_length(precision)
        }
    }
}

impl<F: Float, P: CurvePoint<F>> Curve<F, P> for Bezier3<F, P> {
    fn value_at(&self, t: F) -> P {
        let three = F::one() + F::one() + F::one();

        let t2 = t * t;
        let t3 = t2 * t;

        let nt = F::one() - t;
        let nt2 = nt * nt;
        let nt3 = nt2 * nt;

        self.0
            .scale(&nt3)
            .add(&self.1.scale(&(three * nt2 * t)))
            .add(&self.2.scale(&(three * nt * t2)).add(&self.3.scale(&t3)))
    }

    fn tangent_at(&self, t: F) -> P {
        let Bezier3(p0, p1, p2, p3) = self;

        let three = F::one() + F::one() + F::one();
        let six = three + three;

        let t2 = t * t;

        let nt = F::one() - t;
        let nt2 = nt * nt;

        let v1 = p1.sub(&p0).scale(&(three * nt2));
        let v2 = p2.sub(&p1).scale(&(six * nt * t));
        let v3 = p3.sub(&p2).scale(&(three * t2));

        v1.add(&v2).add(&v3)
    }

    fn start_point(&self) -> P {
        self.0.clone()
    }

    fn end_point(&self) -> P {
        self.3.clone()
    }

    fn estimate_length(&self, precision: F) -> F
    where
        P: Distance<F>,
    {
        let Bezier3(p1, p2, p3, p4) = self;

        let min = p1.distance(p4);
        let max = p1.distance(p2) + p2.distance(p3) + p3.distance(p4);

        let half = F::one() / (F::one() + F::one());

        if max == F::zero() {
            F::zero()
        } else if (max - min) / max < precision {
            (min + max) * half;
        } else {
            let m01 = p1.add(&p2).scale(half);
            let m12 = p2.add(&p3).scale(half);
            let m23 = p3.add(&p4).scale(half);
            let m012 = m01.add(&m12).scale(half);
            let m123 = m12.add(&m23).scale(half);
            let m = m012.add(&m123).scale(half);

            let b1 = Bezier3(p1, m01, m012, m.clone());
            let b2 = Bezier3(m, m123, m23, p4);

            b1.estimate_length(precision) + b2.estimate_length(precision)
        }
    }
}

impl<F: Float, P: CurvePoint<F>> Curve<F, P> for Bezier<F, P> {
    fn value_at(&self, t: F) -> P {
        self.content().value_at(t)
    }

    fn tangent_at(&self, t: F) -> P {
        self.content().tangent_at(t)
    }

    fn start_point(&self) -> P {
        self.content().start_point()
    }

    fn end_point(&self) -> P {
        self.content().end_point()
    }

    fn estimate_length(&self, precision: F) -> F
    where
        P: Distance<F>,
    {
        self.content().estimate_length(precision)
    }
}
