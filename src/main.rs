#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
 loop{
 print!("$ ");
 io::stdout().flush().unwrap();
 let mut s=String::new();
 io::stdin().read_line(&mut s).unwrap();
 let v: Vec<&str>=s.trim().split_whitespace().collect();
 if v.is_empty(){
 continue;
}
 let start = v[0];
 let args =&v[1..];
match start{
 "type" =>{
 let cmd= args[0];
 match cmd{
  "echo" | "exit" | "type" =>{
 println!("{cmd} is a shell builtin");
}

_=>println!("{cmd}: not found"),
}
}
"echo" => {
println!("{}",args.join(" "));
}
"exit" =>{
break;
}
_=>println!("{start}: not found"),
}
}
}
