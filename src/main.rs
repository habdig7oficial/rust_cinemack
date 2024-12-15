use std::io::*;

fn input(msg: Option<&str>) -> String {
  match msg {
    Some(value) => print!("{value}"),
    None => ()
  }

  let mut res = String::new();
   let _ =stdout().flush();
  let _ = stdin().read_line(&mut res).unwrap();
  return res;
}

fn main() {
  let v = input(None);
    println!("{v}");
    
    let v = input(Some("Hello "));
    println!("{v}");
}
