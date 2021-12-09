mod exercise_2_1;
mod exercise_2_2;
mod linked_list;

fn main() {
    {
        // Exercise 2.1
        let mut list = linked_list::List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);

        println!("List before remove_duplicates: {}", list);
        exercise_2_1::remove_duplicates(&mut list);
        println!("List after remove_duplicates: {}", list);
    }
    {
        // Exercise 2.1 — alternative
        let before = linked_list::List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);

        println!("List before remove_duplicates: {}", before);
        let after = exercise_2_1::remove_duplicates2(&before);
        println!("List after remove_duplicates: {}", after);
    }
}
