//mod demo_str;
//mod demo_string;
fn main() {
    // let mut a = Point {x: 0, y: 0};
    // dbg!(&a);
    // a.x = 10;
    // dbg!(&a);
    // mutable_take_ownership(a);
    // dbg!(&a);
    //println!("Hello World");
    //demo_str::run();
    //demo_string::run();
    //run();

    let grap
}


#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

fn mutable_take_ownership(mut a: Point) {
    dbg!(&a);
    a.x += 100;
    a.y += 100;
    dbg!(&a);
}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
//     name: String,
// }

// fn run() {
//     // let mut red = Color {red: 255, green: 0, blue: 0, name: "red".to_string()};
//     // dbg!(&red);
//     // red.red = 250;
//     // dbg!(&red);

//     // let mut blue = get_blue();
//     // dbg!(&blue);
//     // blue.blue = 250;
//     // dbg!(&blue);

//     let colors = get_colors();
//     dbg!(&colors);
//     dbg!(&colors[0]);
//     let colors :Vec<Color> = colors.into_iter().map(|mut color| { if color.name == "blue" { color.blue = 200 }; color}).collect();
//     //&colors[0].blue = 255;
//     dbg!(&colors);
// }

// // fn get_blue() -> Color {
// //     Color {red: 0, green: 0, blue: 255, name: "blue".to_string()}
// // }

// fn get_colors() -> Vec<Color> {
//     let colors = vec![
//         Color {red: 255, green: 0, blue: 0, name: "red".to_string()},
//         Color {red: 0, green: 0, blue: 255, name: "blue".to_string()},
//     ];
//     colors
// }