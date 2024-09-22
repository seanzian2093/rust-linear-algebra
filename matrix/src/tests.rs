// This tests mod is within src so is condidered unittest
#[cfg(test)]
mod tests {
    use crate::Matrix;
    #[test]
    fn test_matrix_new() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        assert_eq!(m.size, 6);
    }
}
