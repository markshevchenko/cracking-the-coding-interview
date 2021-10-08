pub fn replace_spaces(s: &mut Vec<char>, source_length: usize) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn replace_spaces_with_on_ho_returns_oh20ho() {
        assert_eq!(test_replace_spaces("oh, no!", "oh,%20no!"), true);
    }

    #[test]
    fn replace_spaces_with_mr_john_smith_returns_mr_20_john_20_smith() {
        assert_eq!(test_replace_spaces("Mr John Smith", "Mr%20John%20Smith"), true);
    }

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

        crate::exercise_1_3::replace_spaces(&mut v, source_length);

        let actual = v.into_iter().collect::<String>();

        actual == target
    }
}