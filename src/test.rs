pub mod test {
    #[macro_export]
    macro_rules! assert {
        ($totest:expr) => (
            println!("Assertion {}: {}", stringify!($totest), $totest);
            if !($totest) {
                println!("Assertion failed, aborting");
                process::exit(-1);
            }
        );
        ($totest:expr, $($totest_also:expr),+) => (
            assert!($totest);
            assert!($($totest_also),+);
        );
    }
}