use std::f64::consts;

//std::cout vibes from c++
fn main(){
    let pi: f64 = consts::PI;
    let x = pi/2.0;
    println!("this is pi: {}\nand this is sin(pi/2): {}", pi, x.sin());
}