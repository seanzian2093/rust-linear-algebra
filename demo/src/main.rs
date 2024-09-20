use matrix::add;
use vector::Vector;
fn main() {
    let size = 3;
    let vec = vec![1, 2, 3];
    let v = Vector::new(size, vec);
    println!("{:#?}", v);

    let n = v.norm();
    println!("{:#?}", n);
    let sum = add(1, 2);
    println!("{sum}");
}
