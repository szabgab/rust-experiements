pub fn run() {
    let fname = "Foo".to_string();
    let lname = "Bar".to_string();
    println!("{fname}");
    println!("{lname}");
    say_hello(&fname);
    // println!("This is {name}");
    let res = combine(&fname, &lname);
    println!("Combined name {res}");
    println!("{fname}");
    println!("{lname}");
}
fn say_hello(name: &str) {
    println!("Hello {name}");
    println!("Hello {name}");
}

fn combine(fname: &str, lname: &str) -> String {
    //format!("{fname} {lname}")
    fname.to_string() + " " + lname
}


// let mut name = "Foo".to_string();
// println!("{name}");
// let lname = "Bar".to_string();
// name.push_str(&lname);
// let other = &name;  // it works here, but we cannot do this before calling push_str!
// println!("{other}");
// println!("{name}");
