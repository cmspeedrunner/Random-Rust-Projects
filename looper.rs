use std::io;
fn main(){
    loop{
    let mut imp = String::new();
    io::stdin().read_line(&mut imp).expect("Error");
    println!("\x1B[35;1;3m{}\x1B[0m", imp.trim());
    }
}
