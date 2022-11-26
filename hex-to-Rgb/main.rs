use num::{BigInt, Num};
fn convert_hex_to_dec(hex_str: &str) -> String {
    BigInt::from_str_radix(hex_str, 16).unwrap().to_string()
}
pub fn convert_rgb(_str0:String) {
    let _str:String=_str0.replace("#","");
   let r:String=_str[0..2].to_string();
   let _r_int:String=convert_hex_to_dec(&r);
   let g:String=_str[2..4].to_string();
   let _g_int:String=convert_hex_to_dec(&g);
   let b:String=_str[4..6].to_string();
   let _b_int:String=convert_hex_to_dec(&b);
   let rgb:String="rgb".to_string()+"("+&_r_int+","+&_g_int+","+&_b_int+")";
   println!("{}",rgb);
}
pub fn get_rgb(_str0:String)->Vec<f32>{
   let _str:String=_str0.replace("#","");
   let r:String=_str[0..2].to_string();
   let _r_int:String=convert_hex_to_dec(&r);
   let g:String=_str[2..4].to_string();
   let _g_int:String=convert_hex_to_dec(&g);
   let b:String=_str[4..6].to_string();
   let _b_int:String=convert_hex_to_dec(&b);
   let rgb:Vec<f32>=vec![_r_int.parse::<f32>().unwrap(),
   _g_int.parse::<f32>().unwrap(),
   _b_int.parse::<f32>().unwrap()];
   return rgb;
}

