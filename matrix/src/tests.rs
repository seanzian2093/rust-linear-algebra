// This tests mod is within src so is considered unittest
#[cfg(test)]
mod tests {
    use vector::Vector;
    use util::*;
    use crate::Matrix;
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
}
