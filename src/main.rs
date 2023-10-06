use skia_safe::{Data, Typeface};

fn main() {}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn get_font_ids(id: &str, data: Vec<u8>) -> bool {
    let font_data = unsafe { Data::new_bytes(data.as_slice()) };
    let typeface = Typeface::from_data(font_data, None)
        .ok_or_else(|| format!("Failed to load font {} from data", id));

    match typeface {
        Ok(typeface) => true,
        Err(e) => false,
    }
}
