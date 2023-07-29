pub fn run() {
    let mut name = "Foo".to_string();
    println!("{name}");
    //let other = &name;
    name.push_str(" Bar");
    //println!("{other}");
    println!("{name}");

    // say_hello(&name);
    // println!("This is {name}");
}
// fn say_hello(name: &str) {
//     println!("Hello {name}");
//     println!("Hello {name}");
// }
