fn main() {
    // Excercise 1.1
    assert_eq!(contains_duplicates(""), false);
    assert_eq!(contains_duplicates("abcdef"), false);
    assert_eq!(contains_duplicates("foo"), true);

    assert_eq!(contains_duplicates2(""), false);
    assert_eq!(contains_duplicates2("abcdef"), false);
    assert_eq!(contains_duplicates2("foo"), true);    
}

use std::collections::HashSet;

fn contains_duplicates(s: &str) -> bool {
    let mut characters = HashSet::new();

    for c in s.chars() {
        if characters.contains(&c) {
            return true
        } else {
            characters.insert(c);
        }
    }

    false
}

fn contains_duplicates2(s: &str) -> bool {
    let mut skip = 1;
    for c1 in s.chars() {
        for c2 in s.chars().skip(skip) {
            if c1 == c2 {
                return true
            }
        }

        skip += 1
    }
    
    false
}
