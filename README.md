# febits
A Rust library containing various features and tools to help make certain tasks easier

---

A library containing tools to help with certain tasks to make things easier.
Primarily the tools will consist of ways to manipulate primitives and other data types.

prim_u32<br>
\-> u32_to_u16: Split a [u32] into a tuple of ([u16], [u16])

prim_str<br>
\-> find_str_exact:  Find an exact [&str] in another [&str] <br>
\-> find_str: Find any [&str] in another [&str]

string_fe<br>
\-> bytes_to_hex_string:  Hex-encodes [u8] into a [String]<br>
\-> index_hex_string_in_hex_string:  Finds a Hex-encoded [String] inside of another Hex-encoded [String]<br>
\-> index_string_in_u8:  Finds a &[String] inside of a set of [u8]

vectors<br>
\-> sort_vec_floats_f32: Sorts a [`Vec<f32>`] into smallest -> biggest order