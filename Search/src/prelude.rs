use std::{io::prelude::*, sync::Arc};
use std::fs::File;
use regex::Regex;
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
    let m_str:String="".to_owned();
    let m_ntool:Tools=Tools{obj: W(m_str)};
    return m_ntool;
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
pub struct TextFormat{
    m_txt:String,
    expr_regex:String,
}
impl TextFormat {
    pub fn find(&self) -> Option<&str> {
        let re = Regex::new(&self.expr_regex).unwrap();
        match re.find(&self.m_txt) {
            Some(mat) => Some(&self.m_txt[mat.start()..mat.end()]),
            None => None,
        }
    }
    pub fn new()->TextFormat{
    let creation=TextFormat{
        m_txt: "".to_owned(),
        expr_regex: "".to_owned(),
    };
    pub fn SetTxt(mut _self: TextFormat, value:&str){
       _self.m_txt = value.to_owned();
    }
    pub fn SetExpr(mut _self: TextFormat, value:&str){
        _self.expr_regex = value.to_owned();
     }
    return creation;
    }
    pub fn findTxt(&mut self, file_path: &str) {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error: Unable to open the file at the given path.");
                return;
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                let re = Regex::new(&self.expr_regex).unwrap();
                match re.find(&contents) {
                    Some(mat) => self.m_txt = contents[mat.start()..mat.end()].to_string(),
                    None => self.m_txt = "".to_string(),
                }
            }
            Err(_) => println!("Error: Unable to read from the file at the given path."),
        }
    }
}
