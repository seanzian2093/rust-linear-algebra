// This tests mod is within src so is considered unittest
// all results are compared against those from scipy and/or numpy
#[cfg(test)]
mod tests {
    use crate::{AddInto, EuclideanDistance, Vector, DotMul, AddScalar};
    use std::ops::{Add, Sub, Mul, Div};

    #[test]
    fn test_vector_new() {
        let v = Vector::new(3, vec![1, 2, 3]);
        assert_eq!(v.size, 3);
    }

    #[test]
    fn test_vector_norm() {
        let v = Vector::new(3, vec![1, 2, 3]);
        let norm = v.norm();
        assert_eq!(norm, 3.7416573867739413);
    }

    #[test]
    fn test_vector_euclidean_distance() {
        let v1 = Vector::new(3, vec![1, 0, 0]);
        let v2 = Vector::new(3, vec![0, 1, 0]);
        let ed = v1.euclidean_distance(&v2);
        assert_eq!(ed, std::f64::consts::SQRT_2);
    }

    #[test]
    fn test_vector_add_f32_f32() {
        let v1: Vector<f32> = Vector::new(3, vec![1.0, 0.0, 0.0]);
        let v2: Vector<f32> = Vector::new(3, vec![0.0, 1.0, 0.0]);
        let sum = v1.add(v2);
        assert_eq!(sum.size, 3);
    }

    #[test]
    fn test_vector_add_i32_i32() {
        let v1: Vector<i32> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i32> = Vector::new(3, vec![0, 1, 0]);
        let sum = v1.add(v2);
        assert_eq!(sum.size, 3);
    }

    #[test]
    fn test_vector_add_i16_i16() {
        let v1: Vector<i16> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i16> = Vector::new(3, vec![0, 1, 0]);
        let sum = v1.add_into(&v2);
        println!("{:?}.add({:?}) yields {:?}", v1, v2, sum);
        assert_eq!(sum.size, 3);
    }

    #[test]
    fn test_vector_add_i8_i8() {
        let v1: Vector<i8> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i8> = Vector::new(3, vec![0, 1, 0]);
        let sum = v1.add(v2);
        assert_eq!(sum.size, 3);
    }

    #[test]
    #[should_panic]
    fn test_vector_wrong_input() {
        let v1: Vector<i8> = Vector::new(3, vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_vector_sub_i8_i8() {
        let v1: Vector<i8> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i8> = Vector::new(3, vec![0, 1, 0]);
        let res = v1.sub(v2);
        assert_eq!(res.size, 3);
    }

    #[test]
    fn test_vector_mul_i8_i8() {
        let v1: Vector<i8> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i8> = Vector::new(3, vec![0, 1, 0]);
        let res = v1.mul(v2);
        let tgt = Vector::new(3, vec![0.0, 0.0, 0.0]);
        assert_eq!(res, tgt);
    }

    #[test]
    fn test_vector_dot_mul_i8_i8() {
        let v1: Vector<i8> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i8> = Vector::new(3, vec![0, 1, 0]);
        let res = v1.dot_mul(&v2);
        let tgt = 0.0;
        assert_eq!(res, tgt);
    }

    #[test]
    fn test_vector_div_i8_i8() {
        let v1: Vector<i8> = Vector::new(3, vec![2, 2, 2]);
        let v2: Vector<i8> = Vector::new(3, vec![1, 1, 1]);
        let res = v1.div(v2);
        let tgt = Vector::new(3, vec![2.0, 2.0, 2.0]);
        assert_eq!(res, tgt);
    }

    #[test]
    fn test_vector_add_scalar() {
        let v1: Vector<i8> = Vector::new(3, vec![2, 2, 2]);
        let s = 2.2;
        let res = v1.add_scalar(s);
        let tgt = Vector::new(3, vec![4.2, 4.2, 4.2]);
        assert_eq!(res, tgt);
    }
}
