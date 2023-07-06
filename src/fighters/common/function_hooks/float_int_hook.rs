use skyline::hooks::{getRegionAddress, Region};

pub static mut FIGHTER_PARAM_INT_OFFSET : usize = 0x4ded80;
pub static mut FIGHTER_PARAM_FLOAT_OFFSET : usize = 0x4dedc0;

pub static mut PARAM_INT_OFFSET : usize = 0x4E53C0;
pub static mut PARAM_FLOAT_OFFSET : usize = 0x4E5380;


fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];


pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            FIGHTER_PARAM_INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FIGHTER_PARAM_FLOAT_OFFSET = offset;
        }

    }
}