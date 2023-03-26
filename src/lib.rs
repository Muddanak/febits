pub mod vectors;
pub mod prim_u32;
pub mod prim_str;
pub mod string_fe;

#[cfg(test)]
mod tests {
    use crate::prim_u32::u32_to_u16;
    use crate::string_fe::{bytes_to_hex_string, index_hex_string_in_hex_string};
    use crate::vectors::sort_vector_floats;

    // File:  prim_u32
    #[test]
    fn test_prim_u32_u32_to_u16() {
        assert_eq!(u32_to_u16(0x10101010), (0x1010, 0x1010));
        assert_eq!(u32_to_u16(0x11112222), (0x1111, 0x2222));
        assert_eq!(u32_to_u16(0xDEADBEEF), (0xDEAD, 0xBEEF));
    }


    // File:  string_fe.rs
    #[test]
    fn test_bytes_to_hex_string() {

        let data = b"This is test and Test data";

        assert_eq!(bytes_to_hex_string(data), "54686973206973207465737420616e6420546573742064617461");
        assert_eq!(bytes_to_hex_string(&data[3..8]), "7320697320");
    }

    #[test]
    fn test_index_hex_string_in_hex_string() {
        let data = b"This is test and Test data";
        let data = bytes_to_hex_string(data);
        let find = bytes_to_hex_string(b"test");
        let find2 = bytes_to_hex_string(b"Test");
        let find3 = bytes_to_hex_string(b"test and Test");

        assert_eq!(index_hex_string_in_hex_string(&data, &find), 8);
        assert_eq!(index_hex_string_in_hex_string(&data, &find2), 17);
        assert_eq!(index_hex_string_in_hex_string(&data, &find3), 8);
    }

    //File vectors.rs
    #[test]
    fn test_sort_vector_floats() {
        let testvec: Vec<f32> = vec![1.2, 2.3, 1.1, 0.5];
        let testvec2: Vec<f32> = vec![11.0, 12.1, 33.7, std::f32::consts::PI];

        assert_eq!(sort_vector_floats(&testvec), vec![0.5, 1.1, 1.2, 2.3]);
    }



}

