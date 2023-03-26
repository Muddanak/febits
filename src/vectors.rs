
/// Returns a sorted [Vec] of [f32] values from smallest -> largest
///
/// # Example
///
/// ```
/// use febits::vectors::sort_vec_floats_f32;
/// let testvec: Vec<f32> = vec![1.2, 2.3, 1.1, 0.5];
///
/// assert_eq!(sort_vec_floats_f32(&testvec), vec![0.5, 1.1, 1.2, 2.3]);
/// ```
///
pub fn sort_vec_floats_f32(v: &Vec<f32>) -> Vec<f32> {
    let mut cvec: Vec<f32> = v.clone();

    cvec.sort_by(|x:&f32, y:&f32| {
        let Some(res) = x.partial_cmp(y) else { panic!("Error in value to sort, unexpected value!") };
        res
    }
    );

    cvec.to_owned()
}