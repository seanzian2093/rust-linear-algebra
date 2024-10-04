/// Right hand side and caller type must be the same
/// the sum type is Vector<f64>
/// f64 cannot convert back to some other types easily, lossless, e.g. i32
/// let user handle from here
pub trait AddInto<Rhs = Self> {
    type Output;
    fn add_into(&self, _: &Rhs) -> Self::Output;
}
/// Calculate Euclidean distance of Self Vector<T> and other Vector<T>
/// Define an associated type for future/user implementation
pub trait EuclideanDistance<Rhs = Self> {
    fn euclidean_distance(&self, _: &Rhs) -> f64;
}

/// Add a scalar of type U to vector, return a Vector<f64>
pub trait AddScalar<T> {
    type Output;
    fn add_scalar(&self, rhs: T) -> Self::Output;
}
/// Dot Multiplication
pub trait DotMul {
    type Rhs;
    fn dot_mul(&self, other: &Self::Rhs) -> f64;
}

/// Mul a scalar of type U to vector, return a Vector<f64>
pub trait MulScalar<T> {
    type Output;
    fn mul_scalar(&self, rhs: T) -> Self::Output;
}

/// Sub a scalar of type U to vector, return a Vector<f64>
pub trait SubScalar<T> {
    type Output;
    fn sub_scalar(&self, rhs: T) -> Self::Output;
}

/// Div a scalar of type U to vector, return a Vector<f64>
pub trait DivScalar<T> {
    type Output;
    fn div_scalar(&self, rhs: T) -> Self::Output;
}
