// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

//#[allow(non_upper_case_globals, clippy::excessive_precision)]
//const PI: f32 = 3.141_592_7;

use std::f32::consts::PI;

//#[allow(clippy::needless_return, clippy::blacklisted_name, clippy::assign_op_pattern, clippy::collapsible_if)]
///Count to 5.
fn count_to_5() -> i32 {
    let mut x = 0;
    loop {
        if x > PI as i32 && x > 5 {
            break;
        }
        x += 1;
    }
    5
}

/// Main function.
///
/// # SPIDER-BADA
///
/// hello everyone, i can count to 5 using function [link](count_to_5)!!!
fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
