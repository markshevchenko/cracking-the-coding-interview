/*
 * Задача 2.1
 *
 * 🡆 Напишите код для удаления дубликатов из несортированного связного списка.
 * Дополнительно: Как вы будете решать задачу, если использовать временный буфер запрещено?
 */

use std::collections::HashSet;
use super::mutable_list::{List};

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
