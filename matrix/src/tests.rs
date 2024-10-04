// This tests mod is within src so is considered unittest
#[cfg(test)]
mod tests {
    use crate::{Matrix, Vector, AddScalar, MulScalar, SubScalar, DivScalar, MatrixProduct};
    use std::ops::{Add, Sub, Mul, Div};

    #[test]
    fn test_matrix_new() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        assert_eq!(m.size, 6);
    }

    #[test]
    fn test_matrix_rows() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        let res = m.get_rows();
        let tgt = vec![
            Vector::new(2, vec![1,2]),
            Vector::new(2, vec![3,4]),
            Vector::new(2, vec![5,6]),
        ];

        assert_eq!(m.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_cols() {
        let data = vec![1, 2, 3, 4, 5, 6,7,8,9,10,11,12,13,14,15];
        let size = 15;
        let shape = (3, 5);
        let m = Matrix::new(size, shape, data);

        let res = m.get_columns();
        let tgt = vec![
            Vector::new(5, vec![1,2,3,4,5]),
            Vector::new(5, vec![6,7,8,9,10]),
            Vector::new(5, vec![11,12,13,14,15]),
        ];

        assert_eq!(m.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_add_scalar() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        let res = m.add_scalar(2);
        let tgt = vec![
            Vector::new(2, vec![3.,4.]),
            Vector::new(2, vec![5.,6.]),
            Vector::new(2, vec![7.,8.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_mul_scalar() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        let res = m.mul_scalar(2);
        let tgt = vec![
            Vector::new(2, vec![2., 4.]),
            Vector::new(2, vec![6., 8.]),
            Vector::new(2, vec![10., 12.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_sub_scalar() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        let res = m.sub_scalar(1);
        let tgt = vec![
            Vector::new(2, vec![0., 1.]),
            Vector::new(2, vec![2., 3.]),
            Vector::new(2, vec![4., 5.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_div_scalar() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        let res = m.div_scalar(0.5);
        let tgt = vec![
            Vector::new(2, vec![2., 4.]),
            Vector::new(2, vec![6., 8.]),
            Vector::new(2, vec![10., 12.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_add() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);
        let m2 = m.clone();

        let res = m.add(m2);
        let tgt = vec![
            Vector::new(2, vec![2.,4.]),
            Vector::new(2, vec![6.,8.]),
            Vector::new(2, vec![10.,12.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_sub() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);
        let m2 = m.clone();

        let res = m.sub(m2);
        let tgt = vec![
            Vector::new(2, vec![0.,0.]),
            Vector::new(2, vec![0.,0.]),
            Vector::new(2, vec![0.,0.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_div() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);
        let m2 = m.clone();

        let res = m.div(m2);
        let tgt = vec![
            Vector::new(2, vec![1.,1.]),
            Vector::new(2, vec![1.,1.]),
            Vector::new(2, vec![1.,1.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_mul() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);
        let m2 = m.clone();

        let res = m.mul(m2);
        let tgt = vec![
            Vector::new(2, vec![1.,4.]),
            Vector::new(2, vec![9.,16.]),
            Vector::new(2, vec![25.,36.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }

    #[test]
    fn test_matrix_product() {
        let m = Matrix::new(6, (3,2), vec![1, 2, 3, 4, 5, 6]);
        let m2 = Matrix::new(6, (2,3), vec![1, 2, 3, 4, 5, 6]);

        let res = m.mat_product(m2);
        let tgt = vec![
            Vector::new(3, vec![9., 12., 15.]),
            Vector::new(3, vec![19., 26., 33.]),
            Vector::new(3, vec![29., 40., 51.]),
        ];

        assert_eq!(res.get_rows(), tgt);
    }
}
