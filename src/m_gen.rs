use na::{DMatrix, DVector};
//use num_derive::FromPrimitive;

fn index_of(v: &DVector<i32>, e: i32) -> usize {
    v.iter().position(|&x| x == e).unwrap()
}

fn my_apply<F>(v: &DVector<i32>, f: F) -> DVector<i32>
where
    F: Fn(i32, usize) -> i32,
{
    let mut result: DVector<i32> = DVector::zeros(v.len());
    for i in 0..v.len() {
        result[i] = f(v[i], i);
    }
    result
}

fn contains_only_unque<T: PartialEq + Copy>(v: &DVector<T>) -> bool {
    let mut acc = vec![];
    for i in v.iter() {
        if acc.contains(i) {
            return false;
        } else {
            acc.push(*i);
        }
    }
    true
}

//given before (a) and after (b) vector representing cubes
//returns a matrix (m) so that, m*a=b
pub fn mgen(a: &DVector<i32>, b: &DVector<i32>) -> DMatrix<i32> {
    //the before and after vectros have to be the same size
    assert_eq!(a.len(), b.len(), "assertion failed: a.len() == b.len()");

    //checks that a and b have unque entries
    assert!(contains_only_unque(&a), "assertion failed: a[i] == j");
    assert!(contains_only_unque(&b), "assertion failed: a[i] == j");

    //this is the result N by N-transformation-binary-permutation matrix
    let mut result: DMatrix<i32> = DMatrix::zeros(a.len(), a.len());

    //goes thought the matrix rows and updates them
    for i in 0..a.len() {
        result.set_row(
            i,
            &my_apply(&DVector::<i32>::zeros(a.len()), |_, j| -> i32 {
                //assert_eq!(a[i], j as i32, "assertion failed: a[i] == j");
                return if j == index_of(a, b[i]) { 1 } else { 0 };
            })
            .transpose(),
        );
    }
    //returns
    result
}
