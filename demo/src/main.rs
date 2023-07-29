fn main() {
    pass_str("Foo");

    let var1 = "Bar";
    pass_str(var1);
    dbg!(var1);
    other1::run_this();
    other2::say_hello();
    say_hello();
}

fn pass_str(name :&str) {
    dbg!(name);
}

mod other1 {
    pub fn run_this() {
        dbg!("other1 run_this");
    }
}

mod other2;
use other2::say_hello;
