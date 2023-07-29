pub fn run() {
    pass_str("Foo");

    let var1 = "Bar";
    pass_str(var1);
    dbg!(var1);
}


fn pass_str(name :&str) {
    dbg!(name);
}
