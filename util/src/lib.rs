mod traits;
pub use traits::*;

/// All self-define operations take ownership of operands by default
/// to be consistent with std::ops:: which takes ownership of operands
pub fn add_f64(lhs: f64, rhs: f64) -> f64 {
    lhs + rhs
}

pub fn sub_f64(lhs: f64, rhs: f64) -> f64 {
    lhs - rhs
}

pub fn sub_sq_f64(lhs: f64, rhs: f64) -> f64 {
    (lhs - rhs).powi(2)
}

pub fn mul_f64(lhs: f64, rhs: f64) -> f64 {
    lhs * rhs
}

pub fn div_f64(lhs: f64, rhs: f64) -> f64 {
    lhs / rhs
}
pub enum BiOpsF64 {
    AddF64,
    SubF64,
    MulF64,
    DivF64,
    SubSqF64,
}

/// A binary operations between Vec<f64> and Vec<f64>
pub fn bi_op_f64(lhs: Vec<f64>, rhs: Vec<f64>, op: BiOpsF64) -> Vec<f64> {
    assert_eq!(lhs.len(), rhs.len(), "lhs and rhs should have the same length but got {} and {}", lhs.len(), rhs.len());
    let op = match op {
        BiOpsF64::AddF64 => add_f64,
        BiOpsF64::SubF64 => sub_f64,
        BiOpsF64::MulF64 => mul_f64,
        BiOpsF64::DivF64 => div_f64,
        BiOpsF64::SubSqF64 => sub_sq_f64,
    };
    let data: Vec<f64> = lhs.into_iter().zip(rhs).map(|(a, b)| op(a, b)).collect();
    data
}

/// A function converts `Vec<T>` to `Vec<f64>`
pub fn to_vec_f64<T>(data: &Vec<T>) -> Vec<f64>
where
    f64: From<T>,
    T: Clone,
{
    let data: Vec<f64> = data.iter().map(|e| f64::from(e.clone())).collect();

    data
}

/// A function that performs operation on a `scalar` and a `Vec<f64>`
pub fn op_scalar_f64<U>(lhs: Vec<f64>, rhs: U, op:BiOpsF64) -> Vec<f64>
where
    f64: From<U>,
    U: Clone,
{
    let op = match op {
        BiOpsF64::AddF64 => add_f64,
        BiOpsF64::SubF64 => sub_f64,
        BiOpsF64::MulF64 => mul_f64,
        BiOpsF64::DivF64 => div_f64,
        BiOpsF64::SubSqF64 => sub_sq_f64,
    };
    let rhs = f64::from(rhs);
    let data: Vec<f64> = lhs.into_iter().map(|e| op(e, rhs.clone())).collect();
    data
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_f64() {
        let result = add_f64(2.0, 2.0);
        let tgt = 4.0;
        assert_eq!(result, tgt);
    }

    #[test]
    fn test_bi_op_f64() {
        let result = bi_op_f64(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], BiOpsF64::AddF64);
        let tgt = vec![2.0, 4.0, 6.0];
        assert_eq!(result, tgt);
    }

    #[test]
    fn test_op_scalar_f64() {
        let result = op_scalar_f64(vec![1.0, 2.0, 3.0], 1.0, BiOpsF64::AddF64);
        let tgt = vec![2.0, 3.0, 4.0];
        assert_eq!(result, tgt);
    }

    #[test]
    #[should_panic]
    fn test_bi_op_f64_panic() {
        let result = bi_op_f64(vec![1.0, 2.0, 3.0], vec![1.0], BiOpsF64::AddF64);
    }
}
