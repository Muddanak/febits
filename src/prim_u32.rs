use std::ops::{BitAnd, Shr};

/// Returns a tuple of (u16, u16) split from a u32 input
///
/// # Example
///
/// ```
/// use febits::prim_u32::u32_to_u16;
/// let input = 0xDEADBEEF;
///
/// assert_eq!(u32_to_u16(input), (0xDEAD, 0xBEEF));
/// assert_eq!(u32_to_u16(0x11112222), (0x1111, 0x2222));
/// ```
///
pub fn u32_to_u16(inp: u32) -> (u16, u16) {
    let high: u16 = inp.bitand(0xFFFF0000).shr(16) as u16;
    let low: u16 = inp.bitand(0x0000FFFF) as u16;
    (high, low)
}