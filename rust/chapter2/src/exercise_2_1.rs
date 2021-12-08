/*
 * Задача 2.1
 *
 * Напишите код для удаления дубликатов из несортированного связного списка.
 * Дополнительно: Как вы будете решать задачу, если использовать временный буфер запрещено?
 */

use std::collections::HashSet;
use super::linked_list::{List, Node};

pub fn remove_duplicates(list: &mut List) {
    let mut current = match list.head.as_mut() {
        Some(current) => current,
        None => {
            return;
        }
    };
    let mut seen = HashSet::new();
    seen.insert(current.value);
    loop {
        let do_move = match current.next.take() {
            Some(next) if seen.insert(next.value) => {
                current.next = Some(next);
                true
            }
            Some(next) => {
                current.next = next.next;
                false
            }
            None => true,
        };

        if do_move {
            current = match current.next.as_mut().take() {
                Some(current) => current,
                None => return,
            };
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

fn is_in_list(value: i32, node: &Node) -> bool {
    if value == node.value {
        true
    } else {
        if let Some(next_node) = &node.next {
            is_in_list(value, next_node)
        } else {
            false
        }
    }
}

fn recursive_remove_duplicates(node: &Node) -> Node {
    match &node.next {
        None => Node { value: node.value, next: None },
        Some(next_node) => if is_in_list(node.value, next_node) {
            recursive_remove_duplicates(next_node)
        } else {
            Node {
                value: node.value,
                next: Some(Box::new(recursive_remove_duplicates(next_node)))
            }
        }
    }
}

pub fn remove_duplicates2(list: &List) -> List {
    match &list.head {
        None => List { head: None },
        Some(node) => List { head: Some(Box::new(recursive_remove_duplicates(node))) },
    }
}

#[test]
fn remove_duplicates2_with121341356_returns123456() {
    let list = List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);
    let expected = List::from_vec(&vec![2, 4, 1, 3, 5, 6]);

    let actual = remove_duplicates2(&list);

    assert_eq!(expected, actual);
}
