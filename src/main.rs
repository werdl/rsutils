mod test;
use std::process;

fn four() -> i32 {
    return 4i32;
}

fn main() {
    test_fn!(four(), 4i32);
}
