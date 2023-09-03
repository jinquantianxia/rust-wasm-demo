use wasm_bindgen::prelude::*;
// use web_sys::console;

// fn console_log(message: String) {
//     console::log_1(&message.into());
// }

#[wasm_bindgen]
pub fn add_range(a: i32, b: i32) -> i64 {
    // console_log(format!("a, b -> {}, {}", a, b));
    let mut sum = 0;
    for i in a..b {
        sum += i as i64;
    }
    sum
}

#[wasm_bindgen]
pub fn grey_filter(arr: &[u8]) -> Vec<u8> {
    let len = arr.len();
    let mut new_arr: Vec<u8> = vec![0; len];
    let mut i = 0;
    while i < len {
        let average = ((arr[i] as f32 + arr[i + 1] as f32 + arr[i + 2] as f32) / 3.0).round() as u8;
        new_arr[i] = average;
        new_arr[i + 1] = average;
        new_arr[i + 2] = average;
        //alpha值统一为255
        new_arr[i + 3] = 255;
        i += 4;
    }
    new_arr
}

#[wasm_bindgen]
pub fn red_filter(arr: &[u8]) -> Vec<u8> {
    let len = arr.len();
    let mut new_arr: Vec<u8> = vec![0; len];
    let mut i = 0;
    while i < len {
        new_arr[i] = arr[i];
        new_arr[i + 1] = 0;
        new_arr[i + 2] = 0;
        //alpha值统一为255
        new_arr[i + 3] = 255;
        i += 4;
    }
    new_arr
}

#[wasm_bindgen]
pub fn reverse_filter(arr: &[u8]) -> Vec<u8> {
    let len = arr.len();
    let mut new_arr: Vec<u8> = vec![0; len];
    let mut i = 0;
    while i < len {
        new_arr[i] = 255 - arr[i];
        new_arr[i + 1] = 255 - arr[i + 1];
        new_arr[i + 2] = 255 - arr[i + 2];
        //alpha值统一为255
        new_arr[i + 3] = 255;
        i += 4;
    }
    new_arr
}
