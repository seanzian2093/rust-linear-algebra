/// # Vector

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

    pub fn to_f64(&self) -> Vec<f64> {
        // either works
        let data: Vec<f64> = self.data.iter().map(|e| f64::from(e.to_owned())).collect();
        // let data: Vec<f64> = self.data.iter().map(|e| f64::from(e.clone())).collect();
        data
    }
    pub fn norm(&self) -> f64 {
        let norm: f64 = self.to_f64().iter().map(|i| i.powf(2.0)).sum();
        norm.sqrt()
    }
}
