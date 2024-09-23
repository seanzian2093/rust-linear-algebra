// use std::fmt::{write, Debug, Display};
// use std::ops::{Add, Sub};
/// # Vector
// import tests to here, as if the mod tests was written here
use util::*;
mod tests;

// Definition
#[derive(Debug, Clone)]
pub struct Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    size: u32,
    data: Vec<T>,
}

// Implement Display

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
    pub fn new(size: u32, data: Vec<T>) -> Self {
        assert_eq!(
            size,
            data.len() as u32,
            "size and shape do not match - {} vs {:?}",
            size,
            data.len() as u32,
        );
        Vector { size, data }
    }

    fn data_to_f64(&self) -> Vec<f64> {
        let data = to_vec_f64(&self.data);
        data
    }

    // return data as a Vec<f64>
    pub fn get_data(&self) -> Vec<f64> {
        self.data_to_f64()
    }

    // norm
    pub fn norm(&self) -> f64 {
        let norm: f64 = self.data_to_f64().iter().map(|i| i.powf(2.0)).sum();
        norm.sqrt()
    }
}

pub trait EuclideanDistance {
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

        let ed = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::SubSqF64)
            .iter()
            .sum::<f64>()
            .sqrt();

        ed
    }
}

// Borrow and operate
trait AddInto {
    // whatever type B and caller type are, the sum type is Vector<f64>
    // - f64 cannot convert back to some other types easily, losslessly, e.g. i32
    // - we could implement ourself but too tedious.
    // - let user handle from here
    type B;
    fn add_into(&self, _: &Self::B) -> Vector<f64>;
}

impl<T> AddInto for Vector<T>
where
    f64: From<T>,
    T: Clone,
{
    type B = Vector<T>;
    fn add_into(&self, other: &Vector<T>) -> Vector<f64> {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::AddF64);

        Vector::<f64> {
            size: self.size,
            data,
        }
    }
}

// implement the Add trait in std::ops
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
