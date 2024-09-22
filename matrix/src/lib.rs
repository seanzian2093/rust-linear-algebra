mod tests;

#[derive(Debug, Clone)]
pub struct Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    size: u32,
    shape: (u32, u32),
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    // constructor
    pub fn new(size: u32, shape: (u32, u32), data: Vec<T>) -> Self {
        assert_eq!(
            size,
            shape.0 * shape.1,
            "size and shape do not match - {} vs {:?}",
            size,
            shape
        );
        Matrix { size, shape, data }
    }

    // data to f64
    fn data_to_f64(&self) -> Vec<f64> {
        // either works
        let data: Vec<f64> = self.data.iter().map(|e| f64::from(e.to_owned())).collect();
        // let data: Vec<f64> = self.data.iter().map(|e| f64::from(e.clone())).collect();
        data
    }
}
