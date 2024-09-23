use util::*;
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

// Display
impl<T> std::fmt::Display for Matrix<T>
where
    f64: From<T>,
    T: Clone,
    Vec<T>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let it: Vec<()> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, n)| {
                if i == 0 {
                    write!(f, "\n[{} ", i);
                } else {
                    if i as u32 % self.shape.0 == 0 {
                        write!(f, "\n {} ", i);
                    } else {
                        if i as u32 == (self.size - 1) {
                            write!(f, "{}]", i);
                        } else {
                            write!(f, "{} ", i);
                        }
                    }
                }
            })
            .collect();

        // for i in 0..self.shape.0 {
        //     write!(f, "{:?}", self.data);
        // }
        Ok(())
    }
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
        let data = to_vec_f64(&self.data);
        data
    }

    // return data as a Vec<f64>
    pub fn get_data(&self) -> Vec<f64> {
        self.data_to_f64()
    }
}
