
extern crate nalgebra as na;

use na::{Vector6, Matrix6};


fn main() {
    let t = Matrix6::<i32>::new(0, 1, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 1,
                                0, 0, 1, 0, 0, 0,
                                1, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 1, 0,
                                0, 0, 0, 1, 0, 0,);


    let c = Vector6::new(0,1,2,3,4,5);


    let r = t * c;
    println!("{}", t);
    println!("{}", c);
    println!("{}", r);
}
