use std::io::prelude::*;
use std::fs::File;
use num::{BigInt, Num};
pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// Personal preference.
pub use std::format as f;
pub struct Tools{obj:W<String>}

impl  Tools {
    pub fn string_vec(string:  String)->Vec<String> {
         let result=vec![string];
         return result;
    }

    pub fn string_to_str<'a>(_str:&'a String)->&str {
        let read_line =&_str[0.._str.len()];
        return read_line;
    }

    pub fn new()->Tools {
     //   let W<String>=
    }

    pub fn load_file(path:&str)->usize{
        let mut file=File::open(path).unwrap();
        let mut string0:String=String::new();
        let text=&file.read_to_string(&mut string0).unwrap();
        return *text;
    }
pub fn convert_hex_to_dec(hex_str: &str) -> String {
    BigInt::from_str_radix(hex_str, 16).unwrap().to_string()
    }

    pub fn convert_rgb(_str0:String)->[f32; 3] {
    let _str:String=_str0.replace("#","");
    let r:String=_str[0..2].to_string();
    let _r_int:String=Self::convert_hex_to_dec(&r);
    let g:String=_str[2..4].to_string();
    let _g_int:String=Self::convert_hex_to_dec(&g);
    let b:String=_str[4..6].to_string();
    let _b_int:String=Self::convert_hex_to_dec(&b);
    let list_int=[_r_int.parse::<f32>().unwrap(),
    _g_int.parse::<f32>().unwrap(),
    _b_int.parse::<f32>().unwrap()];
    return list_int;
    }

    pub fn get_rgb(_str0:String)->Vec<f32> {
    let _str:String=_str0.replace("#","");
    let r:String=_str[0..2].to_string();
    let _r_int:String=Self::convert_hex_to_dec(&r);
    let g:String=_str[2..4].to_string();
    let _g_int:String=Self::convert_hex_to_dec(&g);
    let b:String=_str[4..6].to_string();
    let _b_int:String=Self::convert_hex_to_dec(&b);
    let rgb:Vec<f32>=vec![_r_int.parse::<f32>().unwrap(),
    _g_int.parse::<f32>().unwrap(),
    _b_int.parse::<f32>().unwrap()];
    return rgb;
    }
}