#[allow(dead_code)]
mod matrix;
mod vector;
use crate::vector::Vector;
use crate::matrix::Matrix;

fn main() {
    let mut vec : Vector<usize, 3> = Vector::new();
    println!("{}", vec.len());
    vec[0] = 12;
    
    println!("{vec}");
    // println!("{}");
    let _a = 
        [[3.0, 0.0, 2.0, 1.0, 0.0],
        [8.0, 2.0, 6.0, 2.0, 0.0],
        [2.0, 6.0, 2.0, 8.0, 4.0],
        [0.0, 6.0, 8.0, 8.0, 8.0],
        [8.0, 5.0, 4.0, 9.0, 8.0]];
    let mat_a = Matrix::construct(_a);
    let trans_a = mat_a.tranpose();
    let inv_a = mat_a.inverse().unwrap();
    println!("{inv_a}");

    let tab_b = [[2.0,3.0,4.0],[2.0,1.0,4.0],[5.0,1.0,0.0]];
    let mat_b = Matrix::construct(tab_b);
    let inv_b = mat_b.inverse().unwrap();
    println!("{inv_b}");


    // let b = [[4.0,1.0,9.0,0.0,8.0],[7.0,9.0,9.0,3.0,0.0],[8.0,4.0,3.0,5.0,0.0],[7.0,6.0,0.0,6.0,7.0],[2.0,5.0,7.0,7.0,1.0]];
    // let m = Matrix::construct(b);

    // println!("{m}");
    // // println!("{:?}", m.to_vec());
    // println!("{:?}", m.det());
    // println!("Hello, world!");
}
