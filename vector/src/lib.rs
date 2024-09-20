/// # Vector
// import tests to here, as if the mod tests was written here
mod tests;

#[derive(Debug)]
pub struct Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    size: usize,
    data: Vec<T>,
}

impl<T> Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    pub fn new(size: usize, data: Vec<T>) -> Self {
        Vector { size, data }
    }

    fn data_to_f64(&self) -> Vec<f64> {
        // either works
        let data: Vec<f64> = self.data.iter().map(|e| f64::from(e.to_owned())).collect();
        // let data: Vec<f64> = self.data.iter().map(|e| f64::from(e.clone())).collect();
        data
    }
    pub fn norm(&self) -> f64 {
        let norm: f64 = self.data_to_f64().iter().map(|i| i.powf(2.0)).sum();
        norm.sqrt()
    }
}

trait EuclideanDistance {
    // we are going to calculate euclidean distance of Self and other Vector<T>
    // use type B to represent generically the other Vector<T>;
    type B;
    fn euclidean_distance(&self, _: &Self::B) -> f64;
}

impl<T> EuclideanDistance for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type B = Vector<T>;
    fn euclidean_distance(&self, other: &Vector<T>) -> f64 {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let ed: f64 = self_data_f64
            .into_iter()
            .zip(other_data_f64)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        ed
    }
}

trait Add {
    // whatever type B and caller type are, the sum type is Vector<f64>
    // - f64 cannot convert back to some other types easily, losslessly, e.g. i32
    // - we could implement ourself but too tedious.
    // - let user handle from here
    type B;
    fn add(&self, _: &Self::B) -> Vector<f64>;
}

impl<T> Add for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type B = Vector<T>;
    fn add(&self, other: &Vector<T>) -> Vector<f64> {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data: Vec<f64> = self_data_f64
            .into_iter()
            .zip(other_data_f64)
            .map(|(a, b)| (a + b))
            .collect();

        // either works - as long as we use turbofish syntax
        // Vector::<f64>::new(self.size, data)
        Vector::<f64> {
            size: self.size,
            data,
        }
    }
}
