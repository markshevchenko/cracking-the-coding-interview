mod exercise_2_1;
mod linked_list;

fn main() {
    let mut list = linked_list::List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);

    exercise_2_1::remove_duplicates(&mut list);

    let mut option_node = &list.head;
    while let Some(node) = option_node {
        println!("{}", node.value);
        option_node = &node.next;
    }
}
