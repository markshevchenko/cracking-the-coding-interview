pub fn is_permutation(s1: &str, s2: &str) -> bool {
    sort_string(s1) == sort_string(s2)
}

fn sort_string(s: &str) -> String {
    let mut v = s.chars().collect::<Vec<_>>();

    v.sort();

    v.into_iter().collect()
}

#[test]
fn is_permutation_with_abc_bac_returns_true() {
    assert_eq!(is_permutation("abc", "bac"), true);
}

#[test]
fn is_permutation_with_aab_bab_returns_true() {
    assert_eq!(is_permutation("aab", "bab"), false);
}

#[test]
fn is_permutation_with_to_be_not_to_be_returns_false() {
    assert_eq!(is_permutation("to be", "not to be"), false);
}
