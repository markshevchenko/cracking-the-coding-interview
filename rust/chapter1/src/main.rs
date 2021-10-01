fn main() {
    // 1.1
    assert_eq!(contains_duplicates(""), false);
    assert_eq!(contains_duplicates("abcdef"), false);
    assert_eq!(contains_duplicates("foo"), true);

    assert_eq!(contains_duplicates2(""), false);
    assert_eq!(contains_duplicates2("abcdef"), false);
    assert_eq!(contains_duplicates2("foo"), true);

    // 1.2
    assert_eq!(are_permutations("abc", "bac"), true);
    assert_eq!(are_permutations("aab", "bab"), false);
    assert_eq!(are_permutations("to be", "not to be"), false);
}

// 1.1
fn contains_duplicates(s: &str) -> bool {
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

// 1.2
fn are_permutations(s1: &str, s2: &str) -> bool {
    sort_string(s1) == sort_string(s2)
}

fn sort_string(s: &str) -> String {
    let mut v = s.chars().collect::<Vec<_>>();
    
    v.sort();

    v.into_iter().collect()
}
