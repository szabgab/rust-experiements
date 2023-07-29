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
    red: u8,
    green: u8,
    blue: u8,
    name: String,
}

fn run() {
    // let mut red = Color {red: 255, green: 0, blue: 0, name: "red".to_string()};
    // dbg!(&red);
    // red.red = 250;
    // dbg!(&red);

    // let mut blue = get_blue();
    // dbg!(&blue);
    // blue.blue = 250;
    // dbg!(&blue);

    let colors = get_colors();
    dbg!(&colors);
}

fn get_blue() -> Color {
    Color {red: 0, green: 0, blue: 255, name: "blue".to_string()}
}

fn get_colors() -> Vec<Color> {
    let colors = vec![
        Color {red: 255, green: 0, blue: 0, name: "red".to_string()},
        Color {red: 0, green: 0, blue: 255, name: "blue".to_string()},
    ];
    colors
}