pub fn run() {
    let name = "Foo".to_string();
    say_hello(name);
}
fn say_hello(name: String) {
    println!("Hello {name}");
}
