mod exercise_1_1;
mod exercise_1_2;
mod exercise_1_3;
mod exercise_1_4;
mod exercise_1_5;
mod exercise_1_6;
mod exercise_1_7;
mod exercise_1_8;
mod exercise_1_9;

fn main() {
    println!("1.1 contains_duplicates(\"abcdef\") -> {}", exercise_1_1::contains_duplicates("abcdef"));
    println!("1.1 contains_duplicates(\"foo\") -> {}", exercise_1_1::contains_duplicates("foo"));
    println!("1.1 contains_duplicates2(\"abcdef\") -> {}", exercise_1_1::contains_duplicates2("abcdef"));
    println!("1.1 contains_duplicates2(\"foo\") -> {}", exercise_1_1::contains_duplicates2("foo"));
    println!();

    println!("1.2 is_permutation(\"abc\", \"bac\") -> {}", exercise_1_2::is_permutation("abc", "bac"));
    println!("1.2 is_permutation(\"to bee\", \"not to be\") -> {}", exercise_1_2::is_permutation("to be", "not to be"));
    println!();

    {
        let mut v = vec!['M', 'r', ' ', 'J', 'o', 'h', 'n', ' ', 'S', 'm', 'i', 't', 'h', ' ', ' ', ' ', ' '];
        println!("1.3 before replace_spaces: {:?}", v);
        exercise_1_3::replace_spaces(&mut v, 13);
        println!("1.3 after replace: spaces: {:?}", v);
        println!();
    }

    println!("1.4 is_permutation_of_palindrome(\"Tact Coa\") -> {}", exercise_1_4::is_permutation_of_palindrome("Tact Coa"));
    println!();

    println!("1.5 is_levenshtein_distance_equals_to_0_or_1(\"pale\", \"ple\") -> {}", exercise_1_5::is_levenshtein_distance_equals_to_0_or_1("pale", "ple"));
    println!("1.5 is_levenshtein_distance_equals_to_0_or_1(\"palex\", \"pale\") -> {}", exercise_1_5::is_levenshtein_distance_equals_to_0_or_1("palex", "pale"));
    println!("1.5 is_levenshtein_distance_equals_to_0_or_1(\"pale\", \"bale\") -> {}", exercise_1_5::is_levenshtein_distance_equals_to_0_or_1("pale", "bale"));
    println!("1.5 is_levenshtein_distance_equals_to_0_or_1(\"pale\", \"bake\") -> {}", exercise_1_5::is_levenshtein_distance_equals_to_0_or_1("pale", "bake"));
    println!();

    println!("1.6 pack(\"abc\") -> {}", exercise_1_6::pack("abc"));
    println!("1.6 pack(\"aabcccccaaa\") -> {}", exercise_1_6::pack("aabcccccaaa"));
    println!();

    {
        let mut v = vec![
            vec![1u32, 2u32],
            vec![3u32, 4u32],
        ];
        println!("1.7 before rotate90:");
        println!("  {:?}", v[0]);
        println!("  {:?}", v[1]);
        println!("1.7 after rotate90:");
        exercise_1_7::rotate90(&mut v);
        println!("  {:?}", v[0]);
        println!("  {:?}", v[1]);
        println!();

        let mut v = vec![
            vec![1u32, 2u32, 3u32],
            vec![4u32, 5u32, 6u32],
            vec![7u32, 8u32, 9u32],
        ];
        println!("1.7 before rotate90:");
        println!("  {:?}", v[0]);
        println!("  {:?}", v[1]);
        println!("  {:?}", v[2]);
        println!("1.7 after rotate90:");
        exercise_1_7::rotate90(&mut v);
        println!("  {:?}", v[0]);
        println!("  {:?}", v[1]);
        println!("  {:?}", v[2]);
        println!();
    }

    {
        let mut v = vec![
            vec![11, 12, 13, 14, 15, 16, 17],
            vec![21,  0, 23, 24, 25, 26, 27],
            vec![31, 32, 33, 34, 35, 36, 37],
            vec![41, 42, 43, 44, 0, 46, 47],
            vec![51, 52, 53, 54, 55, 56, 57],
        ];

        println!("1.8 before zero_rows_and_columns_with_zero:");
        println!("  {:?}", v[0]);
        println!("  {:?}", v[1]);
        println!("  {:?}", v[2]);
        println!("  {:?}", v[3]);
        println!("  {:?}", v[4]);
        println!("1.8 after zero_rows_and_columns_with_zero:");
        exercise_1_8::zero_rows_and_columns_with_zero(&mut v);
        println!("  {:?}", v[0]);
        println!("  {:?}", v[1]);
        println!("  {:?}", v[2]);
        println!("  {:?}", v[3]);
        println!("  {:?}", v[4]);
        println!();
    }

    println!("1.9 is_rotation(\"erbottlewat\", \"waterbottle\") -> {}", exercise_1_9::is_rotation("erbottlewat", "waterbottle"));
    println!();
}
