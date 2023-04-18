use nalgebra::{Vector6, Matrix6};

use crate::colors::{Color as C, Conversions};





#[derive(Debug, Clone)]
pub struct Cube111
{
    vals: Vector6<C>
}

impl Cube111
{
    pub fn new() -> Self
    {
        Cube111 { vals: Vector6::new(C::W,C::B,C::O,C::G,C::R,C::Y) }
    }
    pub fn x(self) -> Self
    {
        let t = Matrix6::<i32>::new( 0, 1, 0, 0, 0, 0,
                                     0, 0, 0, 0, 0, 1,
                                     0, 0, 1, 0, 0, 0,
                                     1, 0, 0, 0, 0, 0,
                                     0, 0, 0, 0, 1, 0,
                                     0, 0, 0, 1, 0, 0, );
        let s1 = t * self.vals.to_numerical();
        println!("{}", s1);
        let s2 = s1.to_colorical();
        println!("{:?}", s2);
        Cube111 { vals: (t * self.vals.to_numerical()).to_colorical() }
    }
    pub fn mx(self) -> Self
    {
        let t = Matrix6::<i32>::new( 0, 0, 0, 1, 0, 0,
                                     1, 0, 0, 0, 0, 0,
                                     0, 0, 1, 0, 0, 0,
                                     0, 0, 0, 0, 0, 1,
                                     0, 0, 0, 0, 1, 0,
                                     0, 1, 0, 0, 0, 0, );
        Cube111 { vals: (t * self.vals.to_numerical()).to_colorical() }
    }
}