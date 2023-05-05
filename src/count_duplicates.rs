/* Count the number of Duplicates
    Write a function that will return the count of distinct case-insensitive alphabetic characters
    and numeric digits that occur more than once in the input string. The input string can be assumed
    to contain only alphabets (both uppercase and lowercase) and numeric digits.
*/

use std::collections::HashMap;

fn count_duplicates(text: &str) -> i32 {
    let mut hash = HashMap::new();
    let mut count = 0;

    for c in text.to_lowercase().chars() {
        match hash.get_mut(&c) {
            Some(count) => {
                *count += 1;
            }
            None => {
                println!("New char {}", c);
                hash.insert(c, 1);
            }
        }
    }

    for (_key, value) in hash.iter() {
        if value >= &2 {
            count += 1;
        }
    }

    count
}

fn main() {
    let text = "Ttt eEEE r P t";

    println!("# of duplicates: {}", count_duplicates(text));
}
