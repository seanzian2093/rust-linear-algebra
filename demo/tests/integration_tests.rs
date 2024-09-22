// integration is at package level and its dir must be next to src
#[cfg(test)]
mod tests {
    use matrix::Matrix;
    use vector::Vector;
    #[test]
    fn test_vector_new() {
        let size = 3;
        let vec = vec![1, 2, 3];
        let v = Vector::new(size, vec);
        println!("{:#?}", v);
    }

    #[test]
    fn test_matrix_new() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let size = 6;
        let shape = (3, 2);
        let m = Matrix::new(size, shape, data);

        println!("{:#?}", m);
    }
}
