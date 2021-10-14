use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn double_previous(val: f64) -> String {
    let int_repr: u64 = u64::from_ne_bytes(val.to_ne_bytes())-1;
    let next_val: f64 = f64::from_ne_bytes(int_repr.to_ne_bytes());
    return std::format!("{}", next_val);
}

#[wasm_bindgen]
pub fn double_next(val: f64) -> String {
    let int_repr: u64 = u64::from_ne_bytes(val.to_ne_bytes())+1;
    let next_val: f64 = f64::from_ne_bytes(int_repr.to_ne_bytes());
    return std::format!("{}", next_val);
}

#[wasm_bindgen]
pub fn float_previous(val: f32) -> String {
    let int_repr: u32 = u32::from_ne_bytes(val.to_ne_bytes())-1;
    let next_val: f32 = f32::from_ne_bytes(int_repr.to_ne_bytes());
    return std::format!("{}", next_val);
}

#[wasm_bindgen]
pub fn float_next(val: f32) -> String {
    let int_repr: u32 = u32::from_ne_bytes(val.to_ne_bytes())+1;
    let next_val: f32 = f32::from_ne_bytes(int_repr.to_ne_bytes());
    return std::format!("{}", next_val);
}
