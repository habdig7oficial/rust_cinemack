use std::io::*;

pub fn input(msg: &str) -> String {
    print!("{msg} ");
    let mut res = String::new();
    let _ =stdout().flush();
    let _ = stdin().read_line(&mut res).unwrap();
    return res;
  }
   
  pub fn rm_endl(input: String) -> String{
    let mut chars = input.chars();
    
    if chars.clone().last().unwrap() == '\n'{
      chars.next_back();
      return chars.into_iter().collect();
    }
    else {
      return input;
    }
    
  }
   
