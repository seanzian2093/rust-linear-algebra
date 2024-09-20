// integration is at package level and its dir must be next to src
#[cfg(test)]
mod tests {
    use matrix::add;
    use vector::Vector;
    #[test]
    fn test_vector_new() {
        let size = 3;
        let vec = vec![1, 2, 3];
        let v = Vector::new(size, vec);
        println!("{:#?}", v);
    }

    #[test]
    fn test_matrix_add() {
        let sum = add(1, 2);
        assert_eq!(sum, 3);
    }
}
