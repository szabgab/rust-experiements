fn main() {
    run("Foo");

    let var1 = "Bar";
    run(var1);
    dbg!(var1);
    other1::run_this();
    other2::run_this();
}

fn run(name :&str) {
    dbg!(name);
}

mod other1 {
    pub fn run_this() {
        dbg!("other1 run_this");
    }
}

mod other2 {
    pub fn run_this() {
        dbg!("other2 run_this");
    }
}
