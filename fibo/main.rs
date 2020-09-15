use std::process; 
use std::io::{self,Write};
use std::fmt::Write as FmtWrite; 

fn main() {
let mut num1: u64 =0 ; 
let mut num2: u64 =1 ;
let mut s = String::new(); 
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    loop {
      let result = num1.checked_add(num2); 
      match result { 
          Some(x) => { num1 = x; write!(handle ,"{}\n", num1);},
          None => { process::exit(0); } 
      } 
      let result =  num2.checked_add(num1); 
      match result { 
          Some(x) => { num2 = x; write!(handle ,"{}\n", num2 );},
          None => { process::exit(0); } 
      } 
    }
}
