use crate::{Curve, Distance, Point};
use num_traits::{One, Zero};
use std::fmt::Debug;

/// Single point
#[derive(Clone, PartialEq)]
pub struct Bezier0<P: Point> {
    pub point: P,
}

impl<P: Point> Bezier0<P> {
    pub fn new(point: P) -> Self {
        Self { point }
    }
}

/// Line
#[derive(Clone, PartialEq)]
pub struct Bezier1<P: Point> {
    pub p0: P,
    pub p1: P,
}

impl<P: Point> Bezier1<P> {
    pub fn new(p0: P, p1: P) -> Self {
        Self { p0, p1 }
    }
}

/// Quadratic bezier curve
#[derive(Clone, PartialEq)]
pub struct Bezier2<P: Point> {
    pub p0: P,
    pub p1: P,
    pub p2: P,
}

impl<P: Point> Bezier2<P> {
    pub fn new(p0: P, p1: P, p2: P) -> Self {
        Self { p0, p1, p2 }
    }
}

/// Cubic bezier curve
#[derive(Clone, PartialEq)]
pub struct Bezier3<P: Point> {
    pub p0: P,
    pub p1: P,
    pub p2: P,
    pub p3: P,
}

impl<P: Point> Bezier3<P> {
    pub fn new(p0: P, p1: P, p2: P, p3: P) -> Self {
        Self { p0, p1, p2, p3 }
    }
}

#[derive(Clone, PartialEq)]
pub enum Bezier<P: Point> {
    C0(Bezier0<P>),
    C1(Bezier1<P>),
    C2(Bezier2<P>),
    C3(Bezier3<P>),
}

impl<P: Point> Copy for Bezier<P> where P: Copy {}
impl<P: Point> Copy for Bezier0<P> where P: Copy {}
impl<P: Point> Copy for Bezier1<P> where P: Copy {}
impl<P: Point> Copy for Bezier2<P> where P: Copy {}
impl<P: Point> Copy for Bezier3<P> where P: Copy {}

macro_rules! for_every_level {
    ($curve:ident, $name:ident, $block:block) => {
        match $curve {
            Bezier::C0($name) => $block,
            Bezier::C1($name) => $block,
            Bezier::C2($name) => $block,
            Bezier::C3($name) => $block,
        }
    };
}

impl<P: Point + Debug> Debug for Bezier<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier")
            .field(for_every_level!(self, c, { c }))
            .finish()
    }
}
impl<P: Point + Debug> Debug for Bezier0<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier0").field(&self.point).finish()
    }
}
impl<P: Point + Debug> Debug for Bezier1<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier1")
            .field(&self.p0)
            .field(&self.p1)
            .finish()
    }
}

impl<P: Point + Debug> Debug for Bezier2<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier2")
            .field(&self.p0)
            .field(&self.p1)
            .field(&self.p2)
            .finish()
    }
}

impl<P: Point + Debug> Debug for Bezier3<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bezier3")
            .field(&self.p0)
            .field(&self.p1)
            .field(&self.p2)
            .field(&self.p3)
            .finish()
    }
}

impl<P: Point> Curve<P> for Bezier0<P> {
    fn value_at(&self, _t: P::Scalar) -> P {
        self.point.clone()
    }

    fn tangent_at(&self, _t: P::Scalar) -> P {
        self.point.scale(P::Scalar::zero())
    }

    fn start_point(&self) -> P {
        self.point.clone()
    }

    fn end_point(&self) -> P {
        self.point.clone()
    }

    fn estimate_length(&self, _precision: P::Scalar) -> P::Scalar
    where
        P: Distance,
    {
        P::Scalar::zero()
    }
}

impl<P: Point> Curve<P> for Bezier1<P> {
    fn value_at(&self, t: P::Scalar) -> P {
        self.p0.add(&self.p1.sub(&self.p0).scale(t))
    }

    fn tangent_at(&self, _t: P::Scalar) -> P {
        self.p1.sub(&self.p0)
    }

    fn start_point(&self) -> P {
        self.p0.clone()
    }

    fn end_point(&self) -> P {
        self.p1.clone()
    }

    fn estimate_length(&self, _precision: P::Scalar) -> P::Scalar
    where
        P: Distance,
    {
        self.p0.distance(&self.p1)
    }
}

impl<P: Point> Curve<P> for Bezier2<P> {
    fn value_at(&self, t: P::Scalar) -> P {
        let t2 = t * t;
        let nt: P::Scalar = P::Scalar::one() - t;
        let nt2 = nt * nt;

        let two = P::Scalar::one() + P::Scalar::one();

        self.p0
            .scale(nt2)
            .add(&self.p1.scale(two * nt * t))
            .add(&self.p2.scale(t2))
    }

    fn tangent_at(&self, t: P::Scalar) -> P {
        let p0 = &self.p0;
        let p1 = &self.p1;
        let p2 = &self.p2;

        let two = P::Scalar::one() + P::Scalar::one();

        let t2: P::Scalar = t + t;
        let nt2 = two - t2;

        let v1 = p1.sub(p0).scale(nt2);
        let v2 = p2.sub(p1).scale(t2);

        v1.add(&v2)
    }

    fn start_point(&self) -> P {
        self.p0.clone()
    }

    fn end_point(&self) -> P {
        self.p2.clone()
    }

    fn estimate_length(&self, precision: P::Scalar) -> P::Scalar
    where
        P: Distance,
    {
        let p0 = &self.p0;
        let p1 = &self.p1;
        let p2 = &self.p2;

        let min: P::Scalar = p0.distance(p1);
        let max: P::Scalar = p0.distance(p1) + p1.distance(p2);

        let half: P::Scalar = P::Scalar::one() / (P::Scalar::one() + P::Scalar::one());

        if max == P::Scalar::zero() {
            P::Scalar::zero()
        } else if (max - min) / max < precision {
            (min + max) * half
        } else {
            let m01 = p0.add(p1).scale(half);
            let m12 = p1.add(p2).scale(half);
            let m = m01.add(&m12).scale(half);

            let b1 = Bezier2::new(p0.clone(), m01, m.clone());
            let b2 = Bezier2::new(m, m12, p2.clone());

            b1.estimate_length(precision) + b2.estimate_length(precision)
        }
    }
}

impl<P: Point> Curve<P> for Bezier3<P> {
    fn value_at(&self, t: P::Scalar) -> P {
        let three = P::Scalar::one() + P::Scalar::one() + P::Scalar::one();

        let t2: P::Scalar = t * t;
        let t3 = t2 * t;

        let nt: P::Scalar = P::Scalar::one() - t;
        let nt2: P::Scalar = nt * nt;
        let nt3 = nt2 * nt;

        self.p0
            .scale(nt3)
            .add(&self.p1.scale(three * nt2 * t))
            .add(&self.p2.scale(three * nt * t2).add(&self.p3.scale(t3)))
    }

    fn tangent_at(&self, t: P::Scalar) -> P {
        let p0 = &self.p0;
        let p1 = &self.p1;
        let p2 = &self.p2;
        let p3 = &self.p3;

        let three = P::Scalar::one() + P::Scalar::one() + P::Scalar::one();
        let six = three + three;

        let t2 = t * t;

        let nt: P::Scalar = P::Scalar::one() - t;
        let nt2 = nt * nt;

        let v1 = p1.sub(p0).scale(three * nt2);
        let v2 = p2.sub(p1).scale(six * nt * t);
        let v3 = p3.sub(p2).scale(three * t2);

        v1.add(&v2).add(&v3)
    }

    fn start_point(&self) -> P {
        self.p0.clone()
    }

    fn end_point(&self) -> P {
        self.p3.clone()
    }

    fn estimate_length(&self, precision: P::Scalar) -> P::Scalar
    where
        P: Distance,
    {
        let p0 = &self.p0;
        let p1 = &self.p1;
        let p2 = &self.p2;
        let p3 = &self.p3;

        let min = p0.distance(p3);
        let max = p0.distance(p1) + p1.distance(p2) + p2.distance(p3);

        let half: P::Scalar = P::Scalar::one() / (P::Scalar::one() + P::Scalar::one());

        if max == P::Scalar::zero() {
            P::Scalar::zero()
        } else if (max - min) / max < precision {
            (min + max) * half
        } else {
            let m01 = p0.add(p1).scale(half);
            let m12 = p1.add(p2).scale(half);
            let m23 = p2.add(p3).scale(half);
            let m012 = m01.add(&m12).scale(half);
            let m123 = m12.add(&m23).scale(half);
            let m = m012.add(&m123).scale(half);

            let b1 = Bezier3::new(p0.clone(), m01, m012, m.clone());
            let b2 = Bezier3::new(m, m123, m23, p3.clone());

            b1.estimate_length(precision) + b2.estimate_length(precision)
        }
    }
}

impl<P: Point> Curve<P> for Bezier<P> {
    fn value_at(&self, t: P::Scalar) -> P {
        for_every_level!(self, c, { c.value_at(t) })
    }

    fn tangent_at(&self, t: P::Scalar) -> P {
        for_every_level!(self, c, { c.tangent_at(t) })
    }

    fn start_point(&self) -> P {
        for_every_level!(self, c, { c.start_point() })
    }

    fn end_point(&self) -> P {
        for_every_level!(self, c, { c.end_point() })
    }

    fn estimate_length(&self, precision: P::Scalar) -> P::Scalar
    where
        P: Distance,
    {
        for_every_level!(self, c, { c.estimate_length(precision) })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bezier_0() {
        let curve = Bezier0::new(2.0);
        assert_eq!(curve.value_at(0.0), 2.0);
        assert_eq!(curve.value_at(0.5), 2.0);
        assert_eq!(curve.value_at(1.0), 2.0);
    }

    #[test]
    fn bezier_1() {
        let curve = Bezier1::new(1.0, 3.0);
        assert_eq!(curve.value_at(0.0), 1.0);
        assert_eq!(curve.value_at(0.5), 2.0);
        assert_eq!(curve.value_at(1.0), 3.0);
    }

    #[test]
    fn bezier_2() {
        let curve = Bezier2::new(1.0, 3.0, 2.0);
        assert_eq!(curve.value_at(0.0), 1.0);
        assert_eq!(curve.value_at(0.5), 2.25);
        assert_eq!(curve.value_at(1.0), 2.0);
    }

    #[test]
    fn bezier_3() {
        let curve = Bezier3::new(1.0, 4.0, 2.0, 4.0);
        assert_eq!(curve.value_at(0.0), 1.0);
        assert_eq!(curve.value_at(0.5), 2.875);
        assert_eq!(curve.value_at(1.0), 4.0);
    }

    #[derive(Clone, PartialEq, Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl Point for Point2D {
        type Scalar = f64;
        fn add(&self, other: &Self) -> Self {
            Point2D {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }

        fn sub(&self, other: &Self) -> Self {
            Point2D {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }

        fn multiply(&self, other: &Self) -> Self {
            Point2D {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }

        fn scale(&self, s: f64) -> Self {
            Point2D {
                x: self.x * s,
                y: self.y * s,
            }
        }
    }

    #[test]
    fn cubic_bezier_2d() {
        let curve = Bezier3::new(
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 0.0, y: 1.0 },
            Point2D { x: 2.0, y: -1.0 },
            Point2D { x: 2.0, y: 0.0 },
        );

        assert_eq!(curve.value_at(0.0), Point2D { x: 0.0, y: 0.0 });
        assert_eq!(curve.value_at(0.5), Point2D { x: 1.0, y: 0.0 });
        assert_eq!(curve.value_at(1.0), Point2D { x: 2.0, y: 0.0 });

        assert_eq!(curve.tangent_at(0.0), Point2D { x: 0.0, y: 3.0 });
        assert_eq!(curve.tangent_at(0.5), Point2D { x: 3.0, y: -1.5 });
        assert_eq!(curve.tangent_at(1.0), Point2D { x: 0.0, y: 3.0 });
    }
}
