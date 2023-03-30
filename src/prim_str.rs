

/// Finds an exact [String] match within a source and a provided pattern
/// and returns the [usize] index pointing to where that pattern is first found.
///
/// Returns [None] if not found
///
/// Takes in:
/// [&str], [&str]
///
/// Outputs:
/// [usize]
///
/// # Examples
///
/// ```
/// use febits::prim_str::find_str_exact;
///
/// let input = "This is Test Data";
/// let pattern = "Test";
/// let index = find_str_exact(input, pattern);
/// assert_eq!(index, Some(8));
/// let index = find_str_exact("This is Test", "is Test");
/// assert_eq!(index, Some(5));
/// let index = find_str_exact("This is test", "Only a test");
/// assert_eq!(index, None);
/// ```
///
pub fn find_str_exact(source: &str, pattern: &str) -> Option<usize> {

    let size = pattern.len();
    'upper: for index in 0..source.len() {
        println!("{index}");
        if source.chars().nth(index) == pattern.chars().nth(0) {
            for pattern_index in 1..size {
                if pattern.chars().nth(pattern_index) != source.chars().nth(index + pattern_index) {
                    continue 'upper
                }
            }
            return Some(index)
        }
    }
    None
}

/// Finds a [String] match within a source and a provided pattern
/// and returns the [usize] index pointing to where that pattern is first found.
///
/// Returns [None] if not found.
///
/// Takes in:
/// [&str], [&str]
///
/// Outputs:
/// [usize]
///
/// # Examples
///
/// ```
/// use febits::prim_str::find_str;
///
/// let input = "THiS Is TeSt Data";
/// let pattern = "TeST";
/// let index = find_str(input, pattern);
/// assert_eq!(index, Some(8));
/// let index = find_str("ThIS iS tESt", "Is tEST");
/// assert_eq!(index, Some(5));
/// let index = find_str("This IS A teST", "JuSt A TeST");
/// assert_eq!(index, None);
/// ```
///
pub fn find_str(source: &str, pattern: &str) -> Option<usize> {

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
            return Some(x)
        }
    }
    None
}