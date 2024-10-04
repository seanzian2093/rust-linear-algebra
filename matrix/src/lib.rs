use util::*;
use vector::Vector;
mod tests;

/// Definition of Matrix struct
#[derive(Debug, Clone)]
pub struct Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    size: usize,
    shape: (usize, usize),
    data: Vec<T>,
}

/// Display
impl<T> std::fmt::Display for Matrix<T>
where
    f64: From<T>,
    T: Clone,
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _: Vec<()> = self
            .data
            .iter()
            .enumerate()
            // cannot use `inspect` because we need to cast, i.e.modify
            .map(|(i, val)| {
                // .map(|(i, val): (usize, &T)| {
                if i == 0 {
                    let _ = write!(f, "\n[{}, ", val);
                } else {
                    if i % self.shape.1 == 0 {
                        let _ = write!(f, "\n {}, ", val);
                    } else {
                        if i == (self.size - 1) {
                            let _ = write!(f, "{}]", val);
                        } else {
                            let _ = write!(f, "{}, ", val);
                        }
                    }
                }
            }).collect();

        Ok(())
    }
}

impl<T> Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    /// constructor
    pub fn new(size: usize, shape: (usize, usize), data: Vec<T>) -> Self {
        assert_eq!(
            size,
            shape.0 * shape.1,
            "size and shape do not match - {} vs {:?}",
            size,
            shape
        );
        Matrix { size, shape, data }
    }

    /// Convert data to Vec<f64>
    fn data_to_f64(&self) -> Vec<f64> {
        let data = to_vec_f64(&self.data);
        data
    }

    /// Return data as a Vec<f64>
    pub fn get_data(&self) -> Vec<f64> {
        self.data_to_f64()
    }

    /// Build rows - more concise way
    fn build_rows(&self) -> Vec<Vector<T>> {
        let mut rows: Vec<Vector<T>> = Vec::new();
        let n_col= self.shape.1;

        // break data in to chunks so each chunk has n_col elements
        for chunk in self.data.chunks(n_col) {
            let v = Vector::new(n_col, chunk.to_owned());
            rows.push(v);
        }
        rows
    }

    /// Build rows - verbose way
    fn _build_rows(&self) -> Vec<Vector<T>> {
        let mut rows: Vec<Vector<T>> = Vec::new();
        let n_col= self.shape.1;
        let n_row= self.shape.0;
        for i in 0..n_row {
            // must use Range<usize> for indexing
            let row_range: std::ops::Range<usize> = (n_col * i..n_col * (i + 1)).into();
            // must use Range<usize> for generic indexing
            let row_data = &self.data[row_range];
            let v = Vector::new(n_col, row_data.to_owned());
            rows.push(v);
        }

        rows
    }

    /// Return rows
    pub fn get_rows(&self) -> Vec<Vector<T>> {
        self.build_rows()
    }

    /// Build columns
    fn build_columns(&self) -> Vec<Vector<T>> {
        let mut cols: Vec<Vector<T>> = Vec::new();
        let n_col= self.shape.1;
        let n_row= self.shape.0;

        // for each column
        for j in 0..n_col{
            // create a data buffer
            let mut col_data = Vec::new();
            // pick element by stepping n_col element and store into buffer
            for stepped in self.data.iter().skip(j).step_by(n_col) {
                col_data.push(stepped.clone());
            }
            // create a vector for this column
            let v = Vector::new(n_row, col_data);
            // push to cols
            cols.push(v);
        }

        cols
    }

    /// Return columns
    pub fn get_columns(&self) -> Vec<Vector<T>> {
        self.build_columns()
    }

}

/// Implement add operation between a `scalar` and a `Matrix<T>`
impl<T, U> AddScalar<U> for Matrix<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Matrix<f64>;
    fn add_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::AddF64);

        Matrix::new(self.size, self.shape, data)
    }
}

/// Implement mul operation between a `scalar` and a `Matrix<T>`
impl<T, U> MulScalar<U> for Matrix<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Matrix<f64>;
    fn mul_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::MulF64);

        Matrix::new(self.size, self.shape, data)
    }
}

/// Implement sub operation between a `scalar` and a `Matrix<T>`
impl<T, U> SubScalar<U> for Matrix<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Matrix<f64>;
    fn sub_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::SubF64);

        Matrix::new(self.size, self.shape, data)
    }
}

/// Implement div operation between a `scalar` and a `Matrix<T>`
impl<T, U> DivScalar<U> for Matrix<T>
where
    f64: From<T>,
    T: Clone,
    f64: From<U>,
    U: Clone,
{
    type Output = Matrix<f64>;
    fn div_scalar(&self, scalar: U) -> Self::Output {
        let self_data_f64 = self.data_to_f64();
        let data = op_scalar_f64(self_data_f64, scalar, BiOpsF64::DivF64);

        Matrix::new(self.size, self.shape, data)
    }
}

/// Implement the in std::ops::Add
impl<T: std::ops::Add<Output = T>> std::ops::Add for Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Matrix<f64>;

    fn add(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::AddF64);
        Matrix::<f64>::new(self.size, self.shape, data)
    }
}

/// Implement the std::ops::Sub
impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Matrix<f64>;

    fn sub(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::SubF64);
        Matrix::<f64>::new(self.size, self.shape, data)
    }
}

/// Implement the std::ops::Div
impl<T: std::ops::Div<Output = T>> std::ops::Div for Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Matrix<f64>;

    fn div(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::DivF64);
        Matrix::<f64>::new(self.size, self.shape, data)
    }
}

/// Implement the std::ops::Mul
impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Matrix<T>
where
    f64: From<T>,
    T: Clone,
{
    type Output = Matrix<f64>;

    fn mul(self, other: Self) -> Self::Output {
        let other_data_f64 = other.data_to_f64();
        let self_data_f64 = self.data_to_f64();

        let data = bi_op_f64(self_data_f64, other_data_f64, BiOpsF64::MulF64);
        Matrix::<f64>::new(self.size, self.shape, data)
    }
}

