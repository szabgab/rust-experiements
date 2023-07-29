pub fn run() {
    let name = "Foo".to_string();
    immutable_take_ownership(name);
    //borrow(&name);
    //let name = give_ownership();
    println!("{name}");
}

fn give_ownership() -> String {
    "Foo".to_string()
}

fn immutable_take_ownership(name: String) {
    println!("{name}");
    //name.push_str(" and bar");
}

fn mutable_take_ownership(mut name: String) {
    name.push_str(" and bar");
    println!("{name}");

}

fn borrow(name: &String) {
    //name.push_str(" and bar");
    println!("{name}");
}

//cases
// we have a variable and we pass it to a function



// pub fn run() {
//     let fname = "Foo".to_string();
//     let lname = "Bar".to_string();
//     println!("{fname}");
//     println!("{lname}");
//     say_hello(&fname);
//     // println!("This is {name}");
//     let res = combine(&fname, &lname);
//     println!("Combined name {res}");
//     println!("{fname}");
//     println!("{lname}");
// }
// fn say_hello(name: &str) {
//     println!("Hello {name}");
//     println!("Hello {name}");
// }

// fn combine(fname: &str, lname: &str) -> String {
//     //format!("{fname} {lname}")
//     fname.to_string() + " " + lname
// }


// let mut name = "Foo".to_string();
// println!("{name}");
// let lname = "Bar".to_string();
// name.push_str(&lname);
// let other = &name;  // it works here, but we cannot do this before calling push_str!
// println!("{other}");
// println!("{name}");
