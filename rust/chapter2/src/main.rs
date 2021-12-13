use crate::exercise_2_2::find_kth_from_end;

mod exercise_2_1;
mod exercise_2_1a;
mod exercise_2_2;
mod mutable_list;
mod immutable_list;

fn main() {
    {
        // Exercise 2.1
        let mut list = mutable_list::List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);

        println!("2.1 List before remove_duplicates: {}", list);
        exercise_2_1::remove_duplicates(&mut list);
        println!("    List after remove_duplicates: {}", list);
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

        println!("2.2 find_kth_from_end({}, 2) is {:?}", &list, find_kth_from_end(&list, 2));
    }
}
