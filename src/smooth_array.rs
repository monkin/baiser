use num_traits::Float;

/// SmoothArray is a data structure that allows to interpolate values between data points.
/// Indexes are in range 0.0..=1.0.
pub(crate) struct SmoothArray<F: Float> {
    data: Vec<F>,
}

impl<F: Float> SmoothArray<F> {
    pub(crate) fn new(data: Vec<F>) -> Self {
        Self { data }
    }

    pub(crate) fn with_steps_count(steps_count: usize) -> Self {
        Self {
            data: vec![F::zero(); steps_count],
        }
    }

    pub(crate) fn value_at(&self, index: F) -> F {
        self.value_at_scaled_index(self.to_array_index(index))
    }

    pub(crate) fn tangent_at(&self, index: F) -> F {
        let i = self.to_array_index(index);
        let v1 = self.value_at_scaled_index(i - F::one());
        let v2 = self.value_at_scaled_index(i + F::one());

        let dl = (F::one() + F::one()) / self.len();

        return (v2 - v1) / dl;
    }

    pub(crate) fn line(&mut self, (i1, v1): (F, F), (i2, v2): (F, F)) {
        let i1 = self.to_array_index(i1);
        let i2 = self.to_array_index(i2);
        let idi = F::one() / (i2 - i1);

        let mut i = i1.ceil();
        let max_i = i2.max(self.len());
        while i < max_i {
            let f = (i - i1) * idi;
            let v = v1 * (F::one() - f) + v2 * f;
            self.data[i.to_usize().unwrap()] = v;
            i = i + F::one();
        }
    }

    fn to_array_index(&self, index: F) -> F {
        index * self.last_index()
    }

    fn value_at_scaled_index(&self, i: F) -> F {
        let i = i.clamp(F::zero(), self.last_index());

        let f = i.fract();
        let i1 = i.floor().to_usize().unwrap();
        let i2 = i.ceil().to_usize().unwrap();

        self.data[i1] * (F::one() - f) + self.data[i2] * f
    }

    fn len(&self) -> F {
        F::from(self.data.len()).unwrap()
    }

    fn last_index(&self) -> F {
        self.len() - F::one()
    }
}
