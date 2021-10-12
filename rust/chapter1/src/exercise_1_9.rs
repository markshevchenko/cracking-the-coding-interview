pub fn is_rotation(s1: &str, s2: &str) -> bool {
    let s1s1 = s1.to_string() + s1;

    s1s1.contains(s2)
}

#[test]
fn is_rotation_with_efabcd_abcdef_returns_true() {
    assert!(is_rotation("efabcd", "abcdef"));
}

#[test]
fn is_rotation_with_123_234_returns_false() {
    assert!(!is_rotation("123", "234"));
}