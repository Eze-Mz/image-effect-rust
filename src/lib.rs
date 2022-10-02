use std::io::Cursor;

//They use keyword allows us to access a specific function or macro from within a crate. We'll be able to reference the macro by its name instead of typing the full path.
use wasm_bindgen::prelude::wasm_bindgen;
//The Web System Crate defines several logging functions. The number refers to how many values can be launched.
//We can add the as keyword to create an alias.
use base64::{decode, encode};
use web_sys::console::log_1 as log;
//The load from memory function will store a copy of the image for manipulation. The goal is to pass our binary data onto the image library for processing.
use image::load_from_memory;
use image::ImageOutputFormat::Png;

//Macro Attributes: a feature for running macro on a function.
//the function expect a string from javascript, the argument will be borrow, thus the "&"
//There are two types of strings. If we're borrowing a string, the type must be str. If we have ownership of a string, the type must be string.
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    //Strings are not equivalent to JavaScript strings. At the end of the day, we're calling a JavaScript function. We need to perform type conversion. This function is one of the most interesting features of rust type conversion can be performed by calling the into function. The into function will intelligently detect the data type where converting to it understands we're trying to convert a string into a js value. There's a lot more going on, but that's the basics of things.
    log(&"grayscale call".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    img = img.grayscale();
    log(&"grayscale effect apply".into());

    //let mut buffer = vec![];
    let mut buffer: Vec<u8> = Vec::new();

    //The right to function will initiate the process of converting an image into binary data.
    //Mutable variables have a different operator for borrowing values:
    img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();
    log(&"New image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    data_url
}
