// This tests mod is within src so is condidered unittest
// all results are compared against those from scipy and/or numpy
#[cfg(test)]
mod tests {
    use crate::{Add, EuclideanDistance, Vector};
    #[test]
    fn test_vector_new() {
        let v = Vector::new(3, vec![1, 2, 3]);
        println!("{:?}", v);
        assert_eq!(v.size, 3);
    }

    #[test]
    fn test_vector_norm() {
        let v = Vector::new(3, vec![1, 2, 3]);
        let norm = v.norm();
        println!("{:?} has a norm of {}", v, norm);
        assert_eq!(norm, 3.7416573867739413);
    }

    #[test]
    fn test_vector_eculidean_distance() {
        let v1 = Vector::new(3, vec![1, 0, 0]);
        let v2 = Vector::new(3, vec![0, 1, 0]);
        let ed = v1.euclidean_distance(&v2);
        println!("{:?} and {:?} has a euclidean distance of {}", v1, v2, ed);
        assert_eq!(ed, 1.4142135623730951);
    }

    #[test]
    fn test_vector_add() {
        let v1: Vector<i32> = Vector::new(3, vec![1, 0, 0]);
        let v2: Vector<i32> = Vector::new(3, vec![0, 1, 0]);
        let sum = v1.add(&v2);
        println!("{:?}.add({:?}) yields {:?}", v1, v2, sum);
        assert_eq!(sum.size, 3);
    }
}
