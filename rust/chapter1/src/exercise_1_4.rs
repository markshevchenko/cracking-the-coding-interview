use std::collections::HashMap;

pub fn is_permutation_of_palindrome(s: &str) -> bool {
    let s: String = s.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let char_counts = count_characters(&s);
    let even_count = char_counts.iter()
                                      .filter(|(_, v)| *v % 2 == 0)
                                      .count();

    even_count == char_counts.len() || even_count == char_counts.len() - 1
}

#[test]
fn is_permutation_palindrome_with_tact_coa_returns_true() {
    assert!(is_permutation_of_palindrome("Tact Coa"));
}

fn count_characters(s: &str) -> HashMap<char, usize> {
    let mut result = HashMap::<char, usize>::new();

    for c in s.chars() {
        if let Some(value) = result.get_mut(&c) {
            *value += 1;
        } else {
            let _ = result.insert(c, 1);
        }
    }

    result
}

#[test]
fn count_characters_with_aaa_returns_a3() {
    let char_counts = count_characters("aaa");

    assert_eq!(1, char_counts.len());
    assert_eq!(3, char_counts[&'a']);
}

#[test]
fn count_characters_with_aabbbcccc_returns_a2_b3_c4() {
    let char_counts = count_characters("aabbbcccc");

    assert_eq!(3, char_counts.len());
    assert_eq!(2, char_counts[&'a']);
    assert_eq!(3, char_counts[&'b']);
    assert_eq!(4, char_counts[&'c']);
}