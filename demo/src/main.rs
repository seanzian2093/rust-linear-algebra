use matrix::*;
use util::*;
use vector::Vector;

fn demo_vector() {
    // vector
    let size = 3;
    let vec = vec![1, 2, 3];
    let v = Vector::new(size, vec);
    println!("\nIn pretty debug: {:#?}", v);
    println!("In debug: {:?}", v);
    println!("In display: {}", v);

    let n = v.norm();
    println!("{:#?}", n);
}

fn demo_matrix() {
    let data = vec![11, 12, 13, 14, 15, 16];
    let size = 6;
    let shape = (3, 2);
    let m = Matrix::new(size, shape, data);

    println!("\nIn pretty debug: {:#?}", m);
    println!("In debug: {:?}", m);
    println!("In display: {}", m);
    println!("Rows: {:?}", m.get_rows());
    println!("Columns: {:?}", m.get_columns());

}

fn demo_matrix2() {
    let data = vec![1, 2, 3, 4, 5, 6,7,8,9,10,11,12,13,14,15, 16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
    let size = 32;
    let shape = (8, 4);
    let m = Matrix::new(size, shape, data);
    println!("\nIn pretty debug: {:#?}", m);
    println!("In debug: {:?}", m);
    println!("In display: {}", m);
    println!("Rows: {:?}", m.get_rows());
    println!("Columns: {:?}", m.get_columns());

}

fn demo_util() {
    let res = add_f64(1.0, 2.0);
    println!("\nadd_f64(1.0, 2.0): {res}");
}

fn main() {
    // demo_matrix();
    demo_matrix2();
}
