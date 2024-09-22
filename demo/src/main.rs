use matrix::*;
use util::*;
use vector::Vector;
fn main() {
    // vector
    let size = 3;
    let vec = vec![1, 2, 3];
    let v = Vector::new(size, vec);
    println!("{:#?}", v);

    let n = v.norm();
    println!("{:#?}", n);

    // matrix
    let data = vec![1, 2, 3, 4, 5, 6];
    let size = 6;
    let shape = (3, 2);
    let m = Matrix::new(size, shape, data);

    println!("{:#?}", m);

    // util
    let res = add_f64(1.0, 2.0);
    println!("{res}");
}
