pub fn run() {
    let mut name = "Foo".to_string();
    println!("{name}");
    let other = &name;
    let lname = "Bar".to_string();
    name.push_str(&lname);
    println!("{other}");
    println!("{name}");

    // say_hello(&name);
    // println!("This is {name}");
}
// fn say_hello(name: &str) {
//     println!("Hello {name}");
//     println!("Hello {name}");
// }
