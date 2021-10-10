pub fn is_levenshtein_distance_equals_to_0_or_1(s1: &str, s2: &str) -> bool {
    let mut previous_row: Vec<usize> = (0..=s2.len()).collect();

    for (i, c1) in s1.chars().enumerate() {
        let mut current_row = vec![i + 1];
        for (j, c2) in s2.chars().enumerate() {
            let insert_cost = current_row[j] + 1;
            let delete_cost = previous_row[j + 1] + 1;
            let replace_cost = previous_row[j] + if c1 == c2 { 0 } else { 1 };

            let distance = min(insert_cost, delete_cost, replace_cost);
            current_row.push(distance);
        }

        previous_row = current_row;
    }

    previous_row[previous_row.len() - 1] < 2
}

#[test]
fn is_levenshtein_distance_equals_to_0_or_1_with_pale_ple_returns_true() {
    assert!(is_levenshtein_distance_equals_to_0_or_1("pale", "ple"));
}

#[test]
fn is_levenshtein_distance_equals_to_0_or_1_with_palex_pale_returns_true() {
    assert!(is_levenshtein_distance_equals_to_0_or_1("palex", "pale"));
}

#[test]
fn is_levenshtein_distance_equals_to_0_or_1_with_pale_bale_returns_true() {
    assert!(is_levenshtein_distance_equals_to_0_or_1("pale", "bale"));
}

#[test]
fn is_levenshtein_distance_equals_to_0_or_1_with_pale_bake_returns_false() {
    assert!(!is_levenshtein_distance_equals_to_0_or_1("pale", "bake"));
}

fn min<T: Ord>(a: T, b: T, c: T) -> T {
    if a < b {
        if a < c {
            a
        } else {
            c
        }
    } else {
        if b < c {
            b
        } else {
            c
        }
    }
}

#[test]
fn min_with_1_2_3_returns_1() {
    assert_eq!(1, min(1, 2, 3));
}

#[test]
fn min_with_2_1_3_returns_1() {
    assert_eq!(1, min(2, 1, 3));
}

#[test]
fn min_with_2_3_1_returns_1() {
    assert_eq!(1, min(2, 3, 1));
}

#[test]
fn min_with_3_1_2_returns_1() {
    assert_eq!(1, min(3, 1, 2));
}