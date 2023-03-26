pub fn sort_vector_floats(v: &Vec<f32>) -> Vec<f32> {
    let mut cvec: Vec<f32> = v.clone();

    cvec.sort_by(|x:&f32, y:&f32| {
        let Some(res) = x.partial_cmp(y) else { panic!("Error in value to sort, unexpected value!") };
        res
    }
    );

    cvec.to_owned()
}