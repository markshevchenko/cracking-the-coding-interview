pub fn contains_duplicates(s: &str) -> bool {
    use std::collections::HashSet;

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

#[test]
fn contains_duplicates_with_empty_string_returns_false() {
    assert_eq!(contains_duplicates(""), false);
}

#[test]
fn contains_duplicates_with_abcdef_returns_false() {
    assert_eq!(contains_duplicates("abcdef"), false);
}

#[test]
fn contains_duplicates_with_foo_returns_true() {
    assert_eq!(contains_duplicates("foo"), true);
}

pub fn contains_duplicates2(s: &str) -> bool {
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

#[test]
fn contains_duplicates2_with_empty_string_returns_false() {
    assert_eq!(contains_duplicates2(""), false);
}

#[test]
fn contains_duplicates2_with_abcdef_returns_false() {
    assert_eq!(contains_duplicates2("abcdef"), false);
}

#[test]
fn contains_duplicates2_with_foo_returns_true() {
    assert_eq!(contains_duplicates2("foo"), true);
}
