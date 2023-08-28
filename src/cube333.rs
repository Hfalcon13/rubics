use nalgebra::{ArrayStorage, Matrix, RowDVector, RowVector, Vector, Vector6, U1, U54};

use crate::colors::{Color as C, Conversions};

pub type Vector54<T> = Vector<T, U54, ArrayStorage<T, 54, 1>>;

struct Cube333 {
    vals: Vector54<u8>,
}

impl Cube333 {
    pub fn new() -> Self {
        Self {
            vals: Matrix::from_vec_generic(
                U54,
                U1,
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
                ],
            ),
        }
    }

    pub fn r(&self) -> Self {
        //top
        let row1 = (0..54).map(|x| if x == 0 { 1 } else { 0 }).collect();
        let row2 = (0..54).map(|x| if x == 1 { 1 } else { 0 }).collect();
        let row3 = (0..54).map(|x| if x == 11 { 1 } else { 0 }).collect();
        let row4 = (0..54).map(|x| if x == 3 { 1 } else { 0 }).collect();
        let row5 = (0..54).map(|x| if x == 4 { 1 } else { 0 }).collect();
        let row6 = (0..54).map(|x| if x == 14 { 1 } else { 0 }).collect();
        let row7 = (0..54).map(|x| if x == 6 { 1 } else { 0 }).collect();
        let row8 = (0..54).map(|x| if x == 7 { 1 } else { 0 }).collect();
        let row9 = (0..54).map(|x| if x == 17 { 1 } else { 0 }).collect();
        //front
        let row10 = (0..54).map(|x| if x == 9 { 1 } else { 0 }).collect();
        let row11 = (0..54).map(|x| if x == 10 { 1 } else { 0 }).collect();
        let row12 = (0..54).map(|x| if x == 47 { 1 } else { 0 }).collect();
        let row13 = (0..54).map(|x| if x == 12 { 1 } else { 0 }).collect();
        let row14 = (0..54).map(|x| if x == 13 { 1 } else { 0 }).collect();
        let row15 = (0..54).map(|x| if x == 50 { 1 } else { 0 }).collect();
        let row16 = (0..54).map(|x| if x == 15 { 1 } else { 0 }).collect();
        let row17 = (0..54).map(|x| if x == 16 { 1 } else { 0 }).collect();
        let row18 = (0..54).map(|x| if x == 53 { 1 } else { 0 }).collect();
        //right
        let row19 = (0..54).map(|x| if x == 24 { 1 } else { 0 }).collect();
        let row20 = (0..54).map(|x| if x == 21 { 1 } else { 0 }).collect();
        let row21 = (0..54).map(|x| if x == 18 { 1 } else { 0 }).collect();
        let row22 = (0..54).map(|x| if x == 25 { 1 } else { 0 }).collect();
        let row23 = (0..54).map(|x| if x == 22 { 1 } else { 0 }).collect();
        let row24 = (0..54).map(|x| if x == 19 { 1 } else { 0 }).collect();
        let row25 = (0..54).map(|x| if x == 26 { 1 } else { 0 }).collect();
        let row26 = (0..54).map(|x| if x == 23 { 1 } else { 0 }).collect();
        let row27 = (0..54).map(|x| if x == 20 { 1 } else { 0 }).collect();
        //back
        let row28 = (0..54).map(|x| if x == 8 { 1 } else { 0 }).collect();
        let row29 = (0..54).map(|x| if x == 28 { 1 } else { 0 }).collect();
        let row30 = (0..54).map(|x| if x == 29 { 1 } else { 0 }).collect();
        let row31 = (0..54).map(|x| if x == 5 { 1 } else { 0 }).collect();
        let row32 = (0..54).map(|x| if x == 31 { 1 } else { 0 }).collect();
        let row33 = (0..54).map(|x| if x == 32 { 1 } else { 0 }).collect();
        let row34 = (0..54).map(|x| if x == 2 { 1 } else { 0 }).collect();
        let row35 = (0..54).map(|x| if x == 34 { 1 } else { 0 }).collect();
        let row36 = (0..54).map(|x| if x == 35 { 1 } else { 0 }).collect();
        //left
        let row37 = (0..54).map(|x| if x == 36 { 1 } else { 0 }).collect();
        let row38 = (0..54).map(|x| if x == 37 { 1 } else { 0 }).collect();
        let row39 = (0..54).map(|x| if x == 38 { 1 } else { 0 }).collect();
        let row40 = (0..54).map(|x| if x == 39 { 1 } else { 0 }).collect();
        let row41 = (0..54).map(|x| if x == 40 { 1 } else { 0 }).collect();
        let row42 = (0..54).map(|x| if x == 41 { 1 } else { 0 }).collect();
        let row43 = (0..54).map(|x| if x == 42 { 1 } else { 0 }).collect();
        let row44 = (0..54).map(|x| if x == 43 { 1 } else { 0 }).collect();
        let row45 = (0..54).map(|x| if x == 44 { 1 } else { 0 }).collect();
        //bottom
        let row46 = (0..54).map(|x| if x == 45 { 1 } else { 0 }).collect();
        let row47 = (0..54).map(|x| if x == 46 { 1 } else { 0 }).collect();
        let row48 = (0..54).map(|x| if x == 33 { 1 } else { 0 }).collect();
        let row49 = (0..54).map(|x| if x == 47 { 1 } else { 0 }).collect();
        let row50 = (0..54).map(|x| if x == 48 { 1 } else { 0 }).collect();
        let row51 = (0..54).map(|x| if x == 30 { 1 } else { 0 }).collect();
        let row52 = (0..54).map(|x| if x == 50 { 1 } else { 0 }).collect();
        let row53 = (0..54).map(|x| if x == 51 { 1 } else { 0 }).collect();
        let row54 = (0..54).map(|x| if x == 27 { 1 } else { 0 }).collect();

        let rows: Vec<Vec<u8>> = vec![
            row1, row2, row3, row4, row5, row6, row7, row8, row9, row10, row11, row12, row13,
            row14, row15, row16, row17, row18, row19, row20, row21, row22, row23, row24, row25,
            row26, row27, row28, row29, row30, row31, row32, row33, row34, row35, row36, row37,
            row38, row39, row40, row41, row42, row43, row44, row45, row46, row47, row48, row49,
            row50, row51, row52, row53, row54,
        ];

        let m: Matrix<u8, U54, U54, ArrayStorage<u8, 54, 54>> =
            Matrix::<u8, U54, U54, ArrayStorage<u8, 54, 54>>::from_vec(rows.concat());

        return Cube333 {
            vals: m * self.vals,
        };
    }
}
