mod exercise_1_1;
mod exercise_1_2;
mod exercise_1_3;
mod exercise_1_4;
mod exercise_1_5;

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
}
