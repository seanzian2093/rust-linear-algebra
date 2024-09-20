use vector::Vector;
fn main() {
    let size = 3;
    let vec = vec![1, 2, 3];
    let v = Vector::new(size, vec);
    println!("{:#?}", v);

    let f = v.to_f64();
    println!("{:#?}", f);

    let n = v.norm();
    println!("{:#?}", n);
}
