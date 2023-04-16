
extern crate nalgebra as na;

use na::{Vector6, Matrix6};

mod colors;

use colors::{Color as C, Conversions};

mod cube111;

use cube111::{Cube111};

fn main() {
    // let t = Matrix6::<i32>::new(0, 1, 0, 0, 0, 0,
    //                             0, 0, 0, 0, 0, 1,
    //                             0, 0, 1, 0, 0, 0,
    //                             1, 0, 0, 0, 0, 0,
    //                             0, 0, 0, 0, 1, 0,
    //                             0, 0, 0, 1, 0, 0,);


    //let c = Vector6::new(C::W,C::B,C::O,C::G,C::R,C::Y);


    //let r = (t * c.to_numerical()).to_colorical();
    //println!("{}", t);
    //!("{}", c);
    //println!("{}", r);

    let c = Cube111::new();



}
