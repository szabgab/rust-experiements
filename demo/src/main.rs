fn main() {
    run("Foo");

    let var1 = "Bar";
    run(var1);
    dbg!(var1);
}

fn run(name :&str) {
    dbg!(name);
}

