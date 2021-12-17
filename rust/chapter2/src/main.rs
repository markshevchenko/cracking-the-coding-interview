mod exercise_2_1;
mod exercise_2_1a;
mod exercise_2_2;
mod exercise_2_3;
mod exercise_2_4;
mod exercise_2_5;
mod exercise_2_5a;
mod exercise_2_6;
mod exercise_2_7;
mod exercise_2_8;
mod mutable_list;
mod immutable_list;

fn main() {
    {
        // Exercise 2.1
        let mut list = mutable_list::List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);

        println!("2.1  List before remove_duplicates: {}", list);
        exercise_2_1::remove_duplicates(&mut list);
        println!("     List after remove_duplicates: {}", list);
    }
    {
        // Exercise 2.1a
        let before = immutable_list::List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);

        println!("2.1a List before remove_duplicates: {}", before);
        let after = exercise_2_1a::remove_duplicates(&before);
        println!("     List after remove_duplicates: {}", after);
    }
    {
        // Exercise 2.2
        let list = immutable_list::List::from_vec(&vec![1, 2, 3, 4, 5, 6]);
        let found_element = exercise_2_2::find_kth_from_end(&list, 2);

        println!("2.2  find_kth_from_end({}, 2) is {:?}", &list, found_element);
    }
    {
        // Exercise 2.3
        let mut list = mutable_list::List::from_vec(&vec![1, 2, 3, 4, 5, 6, 7, 8]);
        println!("2.3  List before remove_from_middle is {}", list);

        exercise_2_3::remove_from_middle(&mut list, 4);

        println!("     List after remove_from_middle with 4 as element to remove: {}", list);
    }
    {
        // Exercise 2.4
        let before = immutable_list::List::from_vec(&vec![3, 2, 1, 10, 5, 8, 5]);
        println!("2.4  List before partial_sort: {}", before);

        let after = exercise_2_4::partial_sort(&before, 5);

        println!("     List after partial_sort with separator 5: {}", after);
    }
    {
        // Exercise 2.5
        let a = immutable_list::List::from_vec(&vec![7, 1, 6]);
        let b = immutable_list::List::from_vec(&vec![5, 9, 2]);
        println!("2.5  List a: {}", a);
        println!("     List b: {}", b);

        let sum = exercise_2_5::sum(&a, &b);
        println!("     sum(&a, &b): {}", sum);
    }
    {
        // Exercise 2.5a
        let a = mutable_list::List::from_vec(&vec![6, 1, 7]);
        let b = mutable_list::List::from_vec(&vec![2, 9, 5]);
        println!("2.5a List a: {}", a);
        println!("     List b: {}", b);

        let sum = exercise_2_5a::sum(a, b);
        println!("     sum(a, b): {}", sum);
    }
    {
        // Exercise 2.6
        let a = immutable_list::List::from_vec(&vec![1, 2, 3, 4, 3, 2, 1]);
        println!("2.6  is_palindrome({}): {}", a, exercise_2_6::is_palindrome(&a));

        let b = immutable_list::List::from_vec(&vec![1, 2, 3, 4, 3, 1, 2]);
        println!("     is_palindrome({}): {}", b, exercise_2_6::is_palindrome(&b));
    }
}
