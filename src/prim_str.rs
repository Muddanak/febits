

/// Finds an exact string match within a source and a provided pattern
/// and returns the usize index pointing to where that pattern is first found
///
/// # Examples
///
/// ```
/// use febits::prim_str::find_str_exact;
///
/// let input = "This is Test Data";
/// let pattern = "Test";
/// let index = find_str_exact(input, pattern);
/// assert_eq!(index, 8);
/// let index = find_str_exact("This is Test", "is Test");
/// assert_eq!(index, 5);
/// ```
///
pub fn find_str_exact(source: &str, pattern: &str) -> usize {

    let size = pattern.len();
    'upper: for index in 0..source.len() {
        println!("{index}");
        if source.chars().nth(index) == pattern.chars().nth(0) {
            for pattern_index in 1..size {
                if pattern.chars().nth(pattern_index) != source.chars().nth(index + pattern_index) {
                    continue 'upper
                }
            }
            return index
        }
    }
    0
}

/// Finds a string match within a source and a provided pattern
/// and returns the usize index pointing to where that pattern is first found
///
/// # Examples
///
/// ```
/// use febits::prim_str::find_str;
///
/// let input = "THiS Is TeSt Data";
/// let pattern = "TeST";
/// let index = find_str(input, pattern);
/// assert_eq!(index, 8);
/// let index = find_str("ThIS iS tESt", "Is tEST");
/// assert_eq!(index, 5);
/// ```
///
pub fn find_str(source: &str, pattern: &str) -> usize {

    let size = pattern.len();
    let data = source.to_lowercase();
    let pattern = pattern.to_lowercase();
    'upper: for x in 0..data.len() {
        if data.chars().nth(x) == pattern.chars().nth(0) {
            for y in 1..size {
                if pattern.chars().nth(y) != data.chars().nth(x+y) {
                    continue 'upper
                }
            }
            return x
        }
    }
    0
}