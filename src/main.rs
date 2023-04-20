
extern crate nalgebra as na;

//use na::{Vector6, Matrix6};

mod colors;

//use colors::{Color as C, Conversions};

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
    assert!(Cube111::new().x().mx() == Cube111::new());
    assert!(Cube111::new().x().x().x().x() == Cube111::new());
    assert!(Cube111::new().x().x() == Cube111::new().mx().mx());
    assert_eq!(Cube111::new().x().y().z(), Cube111::new().x().x().y());
    assert_eq!(Cube111::new(), Cube111::new()
    .x().x().x().x()
    .mx().mx().mx().mx()
    .y().y().y().y()
    .my().my().my().my()
    .z().z().z().z()
    .mz().mz().mz().mz());


    let c1 = Cube111::new();
    println!("main c1 {:?}", c1);
    
    let c2 = Cube111::new();
    println!("main c2 {:?}", c2);



}
