

use num::{BigInt, Num};
fn convert_hex_to_dec(hex_str: &str) -> String {
    BigInt::from_str_radix(hex_str, 16).unwrap().to_string()
}
fn zain(_str0:String) {
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

fn main() 
{  let  runtime:bool =true;
    while  runtime!=false {
        let mut user_input = String::new();
        print!("set a hexcolor value\n");
       std::io::stdin().read_line(&mut user_input).expect("expected a hexcolor value");
        zain(user_input);
       
}
    
}
