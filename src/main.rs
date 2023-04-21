
extern crate nalgebra as na;

use na::{DVector};

mod colors;

//use colors::{Color as C, Conversions};

mod cube111;

use cube111::{Cube111};

mod m_gen;

fn main() {

    


    let c1 = Cube111::new();
    println!("main c1 {:?}", c1);
    
    let c2 = Cube111::new()
    .x().z().z().my().mz().mx().y().mz();
    println!("main c2 {:?}", c2);




}

#[cfg(test)]
pub mod tests
{
    use super::*;

    #[test]
    pub fn cube111_test()
    {
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
    }

    #[test]
    pub fn m_gen_test()
    {
        //let _ = m_gen::addv();
    }

}



