/*
 * Задача 2.1
 *
 * Напишите код для удаления дубликатов из несортированного связного списка.
 * Дополнительно: Как вы будете решать задачу, если использовать временный буфер запрещено?
 */

use std::collections::HashSet;
use super::linked_list::{List};

pub fn remove_duplicates(list: &mut List) {
    let mut current = &mut list.head;
    let mut already = HashSet::new();

    if let Some(first_node) = current {
        already.insert(first_node.value);
    } else {
        return;
    }

    while let Some(current_node) = current {
        let mut do_move = true;
        current_node.next = match current_node.next.take() {
            Some(next_node) if already.contains(&next_node.value) => {
                do_move = false;
                next_node.next
            },
            Some(next_node) => {
                already.insert(next_node.value);

                Some(next_node)
            },
            None => None,
        };

        if do_move {
            current = &mut current_node.next;
        }
    }
}

#[test]
fn remove_duplicates_with121341356_returns123456() {
    let mut list = List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);
    let expected = List::from_vec(&vec![1, 2, 3, 4, 5, 6]);

    remove_duplicates(&mut list);

    assert_eq!(expected, list);
}