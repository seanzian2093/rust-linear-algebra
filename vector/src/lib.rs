/// # Vector
// import tests to here, as if the mod tests was written here
use util::*;
mod tests;

/// Definition of Vector
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    size: usize,
    data: Vec<T>,
}

/// Implement Display

impl<T> std::fmt::Display for Vector<T>
where
    f64: From<T>,
    T: Clone,
    Vec<T>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T> Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    ///
    ///
    /// # Arguments
    ///
    /// * `size`:
    /// * `data`:
    ///
    /// returns: Vector<T>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(size: usize, data: Vec<T>) -> Self {
        assert_eq!(
            size,
            data.len(),
            "size and shape do not match - {} vs {:?}",
            size,
            data.len(),
        );
        Vector { size, data }
    }

    fn data_to_f64(&self) -> Vec<f64> {
        let data = to_vec_f64(&self.data);
        data
    }

    /// return data as a Vec<f64>
    pub fn get_data(&self) -> Vec<f64> {
        self.data_to_f64()
    }

    /// norm
    pub fn norm(&self) -> f64 {
        self.data_to_f64().iter().fold(0.0, |acc, &x| acc + x * x).sqrt()
    }
}

impl<T> EuclideanDistance for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    fn euclidean_distance(&self, other: &Self) -> f64 {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let ed = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::SubSqF64)
            .iter()
            .sum::<f64>()
            .sqrt();

        ed
    }
}

/// Borrow and add

impl<T> AddInto for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    // type Rhs = Vector<T>;
    type Output = Vector<f64>;
    fn add_into(&self, other: &Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();
        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::AddF64);
        Vector::<f64>::new(self.size, data)
    }
}

/// Implement the Add trait in std::ops
impl<T: std::ops::Add<Output = T>> std::ops::Add for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Vector<f64>;

    fn add(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::AddF64);
        Vector::<f64>::new(self.size, data)
    }
}

/// Implement add operation between a `scalar` and a `vector<T>`
impl<T, U> AddScalar<U> for Vector<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Vector<f64>;
    fn add_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::AddF64);

        Vector::<f64>::new(self.size, data)
    }
}

/// Implement mul operation between a `scalar` and a `vector<T>`
impl<T, U> MulScalar<U> for Vector<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Vector<f64>;
    fn mul_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::MulF64);

        Vector::<f64>::new(self.size, data)
    }
}

/// Implement sub operation between a `scalar` and a `vector<T>`
impl<T, U> SubScalar<U> for Vector<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Vector<f64>;
    fn sub_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::SubF64);

        Vector::<f64>::new(self.size, data)
    }
}

/// Implement div operation between a `scalar` and a `vector<T>`
impl<T, U> DivScalar<U> for Vector<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Vector<f64>;
    fn div_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::DivF64);

        Vector::<f64>::new(self.size, data)
    }
}

/// Implement the Sub trait in std::ops
impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Vector<f64>;
    fn sub(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();
        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::SubF64);
        Vector::<f64>::new(self.size, data)
    }
}

/// Implement Mul trait in std::ops
impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Vector<f64>;
    fn mul(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();
        let data=bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::MulF64);
        Vector::<f64>::new(self.size, data)
    }
}

/// Implement std::ops::Dvi
impl<T: std::ops::Div<Output = T>> std::ops::Div for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Vector<f64>;
    fn div(self, rhs: Self) -> Self::Output {
        let other_data_f64 = rhs.data_to_f64();
        let self_data_f64 = self.data_to_f64();
        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::DivF64);
        Vector::<f64>::new(self.size, data)
    }
}

impl<T> VectorProduct for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type Rhs= Vector<T>;
    fn dot_mul(&self, other: &Self::Rhs) -> f64 {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();
        let data=bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::MulF64).into_iter().sum::<f64>();
        data
    }
}