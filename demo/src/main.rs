fn main() {
    run("Foo");

    let var1 = "Bar";
    run(var1);
    dbg!(var1);
}

fn run(name :&str) {
    dbg!(name);
}

mod other {
    fn run_this() {
        dbg!("run_this");
    }
}
