

use core::ops::Mul;

use na::{Matrix, Const, ArrayStorage, U6, U1};
use nalgebra::Vector;
use num_derive::FromPrimitive;


#[derive(Clone, Copy, FromPrimitive, Debug, PartialEq)]
pub enum Color
{
    W = 0,
    B = 1,
    O = 2,
    G = 3,
    R = 4,
    Y = 5,
}

pub trait Conversions
{
    fn to_numerical(self) -> Vector<i32, U6, ArrayStorage<i32, 6, 1>>;
    fn to_colorical(self) -> Vector<Color, U6, ArrayStorage<Color, 6, 1>>;
}

impl Conversions for Matrix<Color, U6, U1, ArrayStorage<Color, 6, 1>>
{
    fn to_numerical(self) -> Vector<i32, U6, ArrayStorage<i32, 6, 1>>
    {
        let mut n = [0; 6];
        for i in 0..=5
        {
            //println!("tn {:?} , i {}", self[i], i);
            n[i] = self[i] as i32;
        }
        Vector::<i32, U6, ArrayStorage<i32, 6, 1>>::new(n[0], n[1], n[2], n[3], n[4], n[5])
    }
    fn to_colorical(self) -> Vector<Color, U6, ArrayStorage<Color, 6, 1>>
    {
        let mut n = [Color::W; 6];
        for i in 0..=5
        {
            n[i] = self[i] as Color;
        }
        Vector::<Color, U6, ArrayStorage<Color, 6, 1>>::new(n[0], n[1], n[2], n[3], n[4], n[5])
    }
}

impl Conversions for Matrix<i32, U6, U1, ArrayStorage<i32, 6, 1>>
{
    fn to_numerical(self) -> Vector<i32, U6, ArrayStorage<i32, 6, 1>>
    {
        let mut n = [0; 6];
        for i in 0..=5
        {
            n[i] = self[i] as i32;
        }
        Vector::<i32, U6, ArrayStorage<i32, 6, 1>>::new(n[0], n[1], n[2], n[3], n[4], n[5])
    }
    fn to_colorical(self) -> Vector<Color, U6, ArrayStorage<Color, 6, 1>>
    {
        let mut n = [Color::W; 6];
        for i in 0..=5
        {
            n[i] = num::FromPrimitive::from_i32(self[i]).unwrap();
        }
        Vector::<Color, U6, ArrayStorage<Color, 6, 1>>::new(n[0], n[1], n[2], n[3], n[4], n[5])
    }
}