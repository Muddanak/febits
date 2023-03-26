pub fn find_str_exact(data: &str, pattern: &str) -> usize {

    let size = pattern.len();
    'upper: for x in 0..data.len() {
        if data.chars().nth(x) == pattern.chars().nth(0) {
            for y in 1..size {
                if pattern.chars().nth(y) != data.chars().nth(x+y) {
                    break 'upper
                }
            }
            return x
        }
    }
    0
}

pub fn find_str(data: &str, pattern: &str) -> usize {

    let size = pattern.len();
    let data = data.to_lowercase();
    let pattern = pattern.to_lowercase();
    'upper: for x in 0..data.len() {
        if data.chars().nth(x) == pattern.chars().nth(0) {
            for y in 1..size {
                if pattern.chars().nth(y) != data.chars().nth(x+y) {
                    break 'upper
                }
            }
            return x
        }
    }
    0
}