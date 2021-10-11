pub fn pack(s: &str) -> String {
    let packed_s = rle(s);

    if packed_s.len() < s.len() {
        packed_s
    } else {
        s.to_string()
    }
}

fn rle(s: &str) -> String {
    let mut result = "".to_string();

    let mut prev_c = '\0';
    let mut count= 0;
    for c in s.chars() {
        if count == 0 {
            prev_c = c;
            count = 1;
        } else if c == prev_c {
            count += 1;
        } else {
            add_char(&mut result, prev_c, count);

            prev_c = c;
            count = 1;
        }
    }

    add_char(&mut result, prev_c, count);
    result
}

#[test]
fn rle_with_empty_string_returns_empty_string() {
    assert_eq!("", rle(""));
}

#[test]
fn rle_with_abc_returns_a1b1c1() {
    assert_eq!("a1b1c1", rle("abc"));
}

#[test]
fn rle_with_aabcccccaaa_returns_a2b1c5a3() {
    assert_eq!("a2b1c5a3", rle("aabcccccaaa"));
}

fn add_char(s: &mut String, c: char, count: usize) {
    if count > 0 {
        s.push(c);
        s.push_str(&count.to_string());
    }
}

#[test]
fn add_char_with_empty_s_c_a_count_1_returns_a1() {
    let mut s = "".to_string();
    add_char(&mut s, 'a', 1);

    assert_eq!("a1", s);
}

#[test]
fn add_char_with_s_a3_c_b_count_2_returns_a3b2() {
    let mut s = "a3".to_string();
    add_char(&mut s, 'b', 2);

    assert_eq!("a3b2", s);
}