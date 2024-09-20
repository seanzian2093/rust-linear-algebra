// This tests mod is within src so is condidered unittest
#[cfg(test)]
mod tests {
    use crate::add;
    #[test]
    fn test_matrix_add() {
        let sum = add(1, 2);
        assert_eq!(sum, 3);
    }
}
