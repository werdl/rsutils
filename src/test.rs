pub mod test {
    #[macro_export]
    macro_rules! assert {
        ($totest:expr) => (
            println!("Assertion {}: {}", stringify!($totest), $totest);
            if !($totest) {
                println!("Assertion failed, aborting");
                std::process::exit(-1);
            }
        );
        ($totest:expr, $($totest_also:expr),+) => (
            assert!($totest);
            assert!($($totest_also),+);
        );
    }



    macro_rules! test_internal {
        ($got:expr, $($expected:expr),+) => {
            if $($got == $expected)||+ {
                true;
            } else {
                false;
            }
        }; 
    }

    pub struct tester {
        passed: i32,
        failed: i32,

        #[macro_export]
        macro_rules! test_fn {
            ($name:literal, $got:expr, $($expected:expr),+) => {
                if $($got == $expected)||+ {
                    println!("Test PASSED - {}", $name);
                } else {
                    println!("Test FAILED - {}", $name);
                }
            };
            ($got:expr, $($expected:expr),+) => {
                if $($got == $expected)||+ {
                    println!("Test PASSED - {}", "anonymous");
                } else {
                    println!("Test FAILED - {}", "anonymous");
                }
            };
        }
    }
}
