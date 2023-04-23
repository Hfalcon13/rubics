
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
        
        let v11: DVector<i32> = na::DVector::from_vec(vec![0,1,2,3,4,5]);
        let v12: DVector<i32> = na::DVector::from_vec(vec![1,5,2,0,4,3]);
        let t1 = m_gen::mgen(&v11, &v12);
        println!("{}", t1);
        assert_eq!(t1, na::Matrix6::<i32>::new( 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0,
            0, 0, 0, 1, 0, 0, ))
    }

}



