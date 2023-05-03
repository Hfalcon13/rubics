
extern crate nalgebra as na;

use na::{DMatrix, DVector, Vector, U24, U1, ArrayStorage};

mod colors;

//use colors::{Color as C, Conversions};

mod cube111;

use cube111::{Cube111};

mod m_gen;

use m_gen::mgen;

fn main() {

    println!("{:?}", Cube111::new());

    let mut c = Cube111::new().x();

    println!("{:?}", c);

    let a = c.solve_depth(4);
    
    println!("{}", a);

    println!("{:?}", c);


    //insted of reprezenting a color by a number, you need to use a RANGE of numbers per color



    // let v1: DVector<i32> = DVector::from_vec(vec![0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5]);
    // println!("{}", v1);
    // let v2: DVector<i32> = DVector::from_vec(vec![0,1,0,1,1,5,1,5,2,2,2,2,0,3,0,3,4,4,4,4,5,3,5,3]);
    // println!("{}", v2);

    // let t = mgen(&v1, &v2);
    // let v3 = t * &v1;
    // println!("{}", v3);
    // println!("{}", &v1 == &v3);
}

#[cfg(test)]
pub mod tests
{
    use super::*;

    use crate::m_gen::mgen;

    

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
    pub fn m_gen_costom_test()
    {
        
        let v11: DVector<i32> = na::DVector::from_vec(vec![0,1,2,3,4,5]);
        let v12: DVector<i32> = na::DVector::from_vec(vec![1,5,2,0,4,3]);
        let t1 = mgen(&v11, &v12);
        println!("{}", t1);
        assert_eq!(t1, na::Matrix6::<i32>::new( 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0,
            0, 0, 0, 1, 0, 0, ))
    }

    #[test]
    fn test_mgen() {
        let a = DVector::from_vec(vec![0, 1, 2, 3, 4]);
        let b = DVector::from_vec(vec![1, 0, 2, 4, 3]);
        let expected_result = DMatrix::from_vec(5, 5, vec![
            0, 1, 0, 0, 0,
            1, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 1,
            0, 0, 0, 1, 0,
        ]);
        let result = mgen(&a, &b);
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "assertion failed: a.len() == b.len()")]
    fn test_mgen_different_sizes() {
        let a = DVector::from_vec(vec![0, 1, 2]);
        let b = DVector::from_vec(vec![1, 0, 2, 4, 3]);
        let _result = mgen(&a, &b);
    }

    #[test]
    #[should_panic(expected = "assertion failed: a[i] == j")]
    fn test_mgen_invalid_permutation() {
        let a = DVector::from_vec(vec![0, 1, 2, 3, 4]);
        let b = DVector::from_vec(vec![1, 0, 2, 4, 4]);
        let _result = mgen(&a, &b);
    }
    #[test]
    fn test_get_fun_by_num() {
        let c = Cube111::new();
        let c_x = c.x();
        let c_mx = c.mx();
        let c_y = c.y();
        let c_my = c.my();
        let c_z = c.z();
        let c_mz = c.mz();

        assert_eq!(Cube111::getFunByNum(0)(&c), c_x);
        assert_eq!(Cube111::getFunByNum(1)(&c), c_mx);
        assert_eq!(Cube111::getFunByNum(2)(&c), c_y);
        assert_eq!(Cube111::getFunByNum(3)(&c), c_my);
        assert_eq!(Cube111::getFunByNum(4)(&c), c_z);
        assert_eq!(Cube111::getFunByNum(5)(&c), c_mz);
    }

    #[test]
    fn test_scrambled() {
        let c = Cube111::scrambled();
        assert_ne!(c, Cube111::new());
    }

    #[test]
    fn test_is_solved() {
        let c = Cube111::new();
        assert!(c.isSolved());

        let c_x = c.x();
        assert!(!c_x.isSolved());
    }

    #[test]
    fn test_solve() {
        let mut c = Cube111::scrambled();
        c.solve();
        assert!(c.isSolved());
    }
}



