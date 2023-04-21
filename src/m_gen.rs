

use na::{ArrayStorage, U6, DVector, Dim, Const, DimName, Vector, Storage, U1};
//use num_derive::FromPrimitive;





fn addv(a: &DVector<i32>, b: &DVector<i32>) -> DVector<i32>
{
    assert_eq!(a.len(), b.len());
    a + b
}



