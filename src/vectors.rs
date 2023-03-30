
/// Returns a mutated [Vec] of [f32] values from smallest -> largest
/// This is an application of the example code provided in the Rust std [slice] library, sort_by
///
/// Takes in:
/// [&mut] [`Vec<f32>`]
///
/// Outputs:
/// Option(`Vec<f32>`)
///
/// # Example
///
/// ```
/// use febits::vectors::sort_vec_floats_f32;
/// let mut testvec: Vec<f32> = vec![1.2, 2.3, 1.1, 0.5];
/// let mut testvec2: Vec<f32> = vec![55.2, 1002.3, 4.4, 99999.2, 123.4, 432.1];
///
/// assert_eq!(sort_vec_floats_f32(&mut testvec), Some(vec![0.5, 1.1, 1.2, 2.3]));
/// assert_eq!(sort_vec_floats_f32(&mut testvec2), Some(vec![4.4, 55.2, 123.4, 432.1, 1002.3, 99999.2]));
/// ```
///
pub fn sort_vec_floats_f32(v: &mut Vec<f32>) -> Option<Vec<f32>> {

    v.sort_by(move |first, second| {
        match first.partial_cmp(second) {
            Some(res) => res,
            None => panic!("Unexpected value encountered in Vector sort f32"),
        }
    });

    Some(v.to_owned())
}