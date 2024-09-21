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
    let sum = add(1, 2);
    println!("{sum}");

    // util
    let res = add_f64(1.0, 2.0);
    println!("{res}");
}
