use nalgebra::{Matrix6, Vector6};

use crate::colors::{Color as C, Conversions};

#[derive(Debug, Clone)]
pub struct Cube111 {
    vals: Vector6<C>,
}

impl PartialEq for Cube111 {
    fn eq(&self, other: &Self) -> bool {
        self.vals == other.vals
    }
}

impl Cube111 {
    pub fn new() -> Self {
        Cube111 {
            vals: Vector6::new(C::W, C::B, C::O, C::G, C::R, C::Y),
        }
    }
    pub fn x(&self) -> Self {
        let t = Matrix6::<i32>::new(
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 1, 0, 0,
        );
        //let s1 = t * self.vals.to_numerical();
        //println!("s1 {}", s1);
        //let s2 = s1.to_colorical();
        //println!("s2 {:?}", s2);
        Cube111 {
            vals: (t * self.vals.to_numerical()).to_colorical(),
        }
    }
    pub fn mx(&self) -> Self {
        let t = Matrix6::<i32>::new(
            0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0, 0,
        );
        Cube111 {
            vals: (t * self.vals.to_numerical()).to_colorical(),
        }
    }
    pub fn y(&self) -> Self {
        let t = Matrix6::<i32>::new(
            1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1,
        );
        Cube111 {
            vals: (t * self.vals.to_numerical()).to_colorical(),
        }
    }
    pub fn my(&self) -> Self {
        let t = Matrix6::<i32>::new(
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1,
        );
        Cube111 {
            vals: (t * self.vals.to_numerical()).to_colorical(),
        }
    }
    pub fn z(&self) -> Self {
        let t = Matrix6::<i32>::new(
            0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 1, 0, 0, 0,
        );
        Cube111 {
            vals: (t * self.vals.to_numerical()).to_colorical(),
        }
    }
    pub fn mz(&self) -> Self {
        let t = Matrix6::<i32>::new(
            0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0,
        );
        Cube111 {
            vals: (t * self.vals.to_numerical()).to_colorical(),
        }
    }
}

impl Cube111 {
    pub fn getFunByNum(n: u8) -> fn(&Cube111) -> Cube111 {
        match n {
            0 => Cube111::x,
            1 => Cube111::mx,
            2 => Cube111::y,
            3 => Cube111::my,
            4 => Cube111::z,
            5 => Cube111::mz,
            _ => panic!("invalid number"),
        }
    }

    pub fn scrambled() -> Cube111 {
        extern crate rand;
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut c = Cube111::new();

        let n: u8 = rng.gen_range(5..20);
        for i in 0..n {
            c = Cube111::getFunByNum(rng.gen_range(0..6))(&c);
        }

        c
    }

    pub fn isSolved(&self) -> bool {
        *self == Cube111::new()
    }

    //goal: make this go a variable amount of depth
    //instead of just 1
    pub fn solve(&mut self) {
        for i in 0..6 {
            if Cube111::getFunByNum(i)(&self).isSolved() {
                *self = Cube111::getFunByNum(i)(&self);
            }
        }
    }
    pub fn solve_depth(&mut self, depth: u8) -> u128 {
        //println!("to fix {:?}", self);
        assert!(depth > 0);

        let mut acc = 0;

        if depth == 1 {
            //println!("got here");
            for i in 0..6 {
                if (Cube111::getFunByNum(i)(&self)).isSolved() {
                    *self = Cube111::getFunByNum(i)(&self);
                    println!("sd  ->  {:?}", Cube111::getFunByNum(i)(&self));
                    return 1;
                }
            }
        } else {
            for i in 0..6 {
                acc += (Cube111::getFunByNum(i)(&self)).solve_depth(depth - 1);
            }
        }
        return acc;
    }
}
