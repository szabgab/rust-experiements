pub fn run() {
    let name = "Foo".to_string();
    println!("{name}");
    let other = &name;
    println!("{other}");
    println!("{name}");

    // say_hello(&name);
    // println!("This is {name}");
}
// fn say_hello(name: &str) {
//     println!("Hello {name}");
//     println!("Hello {name}");
// }
