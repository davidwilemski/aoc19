use std::collections::HashMap;

fn main() {
    // range: 265275-781584
    //
    for i in 265275..=781584 {
        // It is a six-digit number.
        // No need to check, already guaranteed

        // Two adjacent digits are the same (like 22 in 122345).
        if !has_adjacent_equal_digits(i) {
            continue;
        }


        // Going from left to right, the digits never decrease; they only ever increase or stay the
        // same
        if !digits_do_not_decrease(i) {
            continue;
        }

        if !digit_counts_exactly_two(i) {
            continue;
        }

        println!("{}", i);

    }
}

fn has_adjacent_equal_digits(val: i32) -> bool {
        let str_val = val.to_string();
        for vals in str_val.chars().collect::<Vec<char>>().windows(2) {
            if vals[0] == vals[1] {
                return true;
            }
        }
        false
}

fn digits_do_not_decrease(val: i32) -> bool {
    let str_val = val.to_string();
    let vals = str_val.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let mut rev_sorted_vals = vals.clone();
    rev_sorted_vals.sort();

    if &vals == &rev_sorted_vals {
        return true;
    }
    false
}

fn digit_counts_exactly_two(val: i32) -> bool {
    let str_val = val.to_string();
    let mut digit_counts = HashMap::new();

    for c in str_val.chars() {
        if let Some(count) = digit_counts.get_mut(&c) {
            *count += 1;
        } else {
            digit_counts.insert(c, 1);
        }
    }

    for v in digit_counts.values() {
        if *v == 2 {
            return true;
        }
    }

    false
}
