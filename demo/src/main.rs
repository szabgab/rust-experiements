fn main() {
    demo_str::run();

    other1::run_this();
    other2::say_hello();
    say_hello();
}


mod other1 {
    pub fn run_this() {
        dbg!("other1 run_this");
    }
}

mod other2;
use other2::say_hello;
mod demo_str;