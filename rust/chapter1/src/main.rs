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

    // 1.3
    assert_eq!(test_replace_spaces("oh, no!", "oh,%20no!"), true);
    assert_eq!(test_replace_spaces("Mr John Smith", "Mr%20John%20Smith"), true);
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

// 1.3
fn test_replace_spaces(source: &str, target: &str) -> bool {
    let mut v = source.chars().collect::<Vec<_>>();
    let source_length = source.len();
    let spaces_count = source.chars()
                             .filter(|&c| c == ' ')
                             .count();

    for _ in 0..spaces_count {
        v.push(' ');
        v.push(' ');
    }

    replace_spaces(&mut v, source_length);

    let actual = v.into_iter().collect::<String>();

    actual == target
}

fn replace_spaces(s: &mut Vec<char>, source_length: usize) {
    for i in (0..source_length).rev() {
        if s[i] == ' ' {
            for j in (i + 1..s.len()).rev() {
                s[j] = s[j - 2]
            }

            s[i] = '%';
            s[i + 1] = '2';
            s[i + 2] = '0';
        }
    }
}