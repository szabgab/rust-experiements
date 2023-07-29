pub fn run() {
    let mut name = "Foo".to_string();
    println!("{name}");
    //let other = &name;
    let lname = "Bar".to_string();
    let x = &name.push_str(&lname);
    //println!("{other}");
    println!("{name}");
    println!("{:?}",x );
    // say_hello(&name);
    // println!("This is {name}");
}
// fn say_hello(name: &str) {
//     println!("Hello {name}");
//     println!("Hello {name}");
// }



// let mut name = "Foo".to_string();
// println!("{name}");
// let lname = "Bar".to_string();
// name.push_str(&lname);
// let other = &name;  // it works here, but we cannot do this before calling push_str!
// println!("{other}");
// println!("{name}");
