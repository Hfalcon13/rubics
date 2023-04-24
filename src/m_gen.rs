


use na::{ArrayStorage, DMatrix, DVector, Dim, DimName, Vector1, Matrix, Const};
//use num_derive::FromPrimitive;

fn index_of(v: &DVector<i32>, e: i32) -> usize
{
    v.iter().position(|&x| x == e).unwrap()
}

fn my_apply<F>(v: &DVector<i32>, f: F) -> DVector<i32>
where
    F: Fn(i32, usize) -> i32,
{
    let mut result: DVector<i32> = DVector::zeros(v.len());
    for i in 0..v.len()
    {
        result[i] = f(v[i], i);
    }
    result
}



pub fn mgen(a: &DVector<i32>, b: &DVector<i32>) -> DMatrix<i32>
{
    //the before and after vectros have to be the same size
    assert_eq!(a.len(), b.len());
    //this is the result N by N-transformation-binary-permutation matrix
    let mut result: DMatrix<i32> = DMatrix::zeros(a.len(), a.len());
    //goes thought the matrix rows and updates them
    for i in 0..a.len()
    {
        result
        .set_row(i, 
            &my_apply(&DVector::<i32>::zeros(a.len()), 
            |_,j|if j == index_of(a, b[i]){1}else{0})
        .transpose());
    }
    //returns
    result
}



