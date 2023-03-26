use std::process::exit;
use std::fmt::Write;

#[allow(dead_code)]
pub fn bytes_to_hex_string(data: &[u8]) -> String {

    let mut buffer = String::new();

    for &x in data {
        if let Err(e) = write!(&mut buffer, "{:x?}", x) {
            println!("Failed to convert byte to hex! Got Error: {e}");
            exit(1)
        }
    }

    buffer
}

#[allow(dead_code)]
pub fn index_hex_string_in_hex_string(data: &String, find: &String) -> usize {

    if !data.contains(find) {
        println!("The string to find was not in the data");
        return 0
    }

    if let Some(index) = data.find(find) {
        return index/2
    }

    0
}