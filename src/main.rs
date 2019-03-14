use std::env;

fn gen_serial (entered_name: &str) -> String
{
 let mut serial = vec![0u8; 10];
 let name: &[u8] = entered_name.as_bytes();
 let mut ctr=0;
 for i in (1..6).rev(){
    let ascii_val: u8 = name[ctr];
    let v1 = (ascii_val ^ 0x29) + (i as u8);
    serial[ctr] = if v1 < b'A' || v1 > b'Z' { 0x52 + (i as u8) } else { v1 };
    let mut v2 = (ascii_val ^ 0x27) + (i as u8);
    v2 +=1;
    serial[ctr+5] = if v2 < b'A' || v2 > b'Z' { 0x4D + (i as u8) } else { v2 };
    ctr+=1;
    }
   
    for i in 0..10{
    let mut gen: u8 = serial[i]+5;
    if gen > b'Z' { gen -= 0xD;}
    gen ^= 0xC;
    if gen < b'A'{ gen = 0x4B + (i as u8); }
    if gen > b'Z' { gen = 0x4B - (i as u8); }
    serial[i] = gen;
    }
    String::from_utf8(serial).expect("Found invalid UTF-8")
}

fn main() {
    let args: Vec<String> = env::args().collect();
     match args.len() {
    1 => {
         println!("Please input your name.");
    }   
    2 =>{
    let username = &args[1];
    println!("Serial: {}", gen_serial(username));
    },
     _ => {
         println!("Please input your name.");
    }
    }   
}
