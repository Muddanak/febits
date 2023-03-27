use std::process::exit;
use std::fmt::Write;

/// Returns a [String] of Hexadecimal-encoded bytes of &[u8] provided
///
/// # Example
///
/// ```
/// use febits::string_fe::bytes_to_hex_string;
///
/// let sample = b"This is rich";
///
/// assert_eq!(bytes_to_hex_string(sample), "546869732069732072696368");
/// assert_eq!(bytes_to_hex_string(b"Other data to hex String"), "4f74686572206461746120746f2068657820537472696e67");
/// ```
///
pub fn bytes_to_hex_string(source: &[u8]) -> String {

    let mut buffer = String::new();

    for &x in source {
        if let Err(e) = write!(&mut buffer, "{:02X?}", x) {
            println!("Failed to convert byte to hex! Got Error: {e}");
            exit(1)
        }
    }

    buffer
}

/// Returns the index of one Hex-encoded [String] pattern located inside of
/// another Hex-encoded [String] source
///
/// # Example
///
/// ```
/// use febits::string_fe::{bytes_to_hex_string, index_hex_string_in_hex_string};
///
/// let source = bytes_to_hex_string(b"This is Rich text");
/// let pattern = bytes_to_hex_string(b"Rich");
///
/// assert_eq!(index_hex_string_in_hex_string(&source, &pattern), 8);
/// ```
///
pub fn index_hex_string_in_hex_string(source: &String, pattern: &String) -> usize {

    if !source.contains(pattern) {
        println!("The string to find was not in the data");
        return 0
    }

    if let Some(index) = source.find(pattern) {
        return index/2
    }

    0
}

/// Find a given [String] pattern inside of a given source of &[u8]
///
/// # Example
///
/// ```
/// use febits::string_fe::index_string_in_u8;
/// let input = b"This is Sample text";
/// let pat = String::from("Sample");
///
/// assert_eq!(index_string_in_u8(input, &pat), 8);
/// ```
///
pub fn index_string_in_u8(source: &[u8], pattern: &String) -> usize {
    let data = bytes_to_hex_string(source);
    let pat = bytes_to_hex_string(pattern.as_bytes());

    index_hex_string_in_hex_string(&data, &pat)
}
