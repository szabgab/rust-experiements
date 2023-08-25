use std::f32::consts::PI;
use std::f64::consts::PI as bigPI;
use std::f64::consts;

fn main() {
    println!("{}", std::f32::consts::PI);
    println!("{}", std::f64::consts::PI);
    println!("{}", PI);
    println!("{}", bigPI);
    println!("{}", consts::PI);
    println!("{}", consts::E);
}
