//mod demo_str;
//mod demo_string;
fn main() {
    //println!("Hello World");
    //demo_str::run();
    //demo_string::run();
    run();
}

#[derive(Debug)]
#[allow(dead_code)]
struct Color {
    mut red: u8,
    green: u8,
    blue: u8,
    name: String,
}

fn run() {
    mut red = Color {red: 255, green: 0, blue: 0, name: "red".to_string()};
    dbg!(red);
    //red.red = 200;
    dbg!(red);
}

