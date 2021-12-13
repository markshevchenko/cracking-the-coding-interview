/*
 * Задача 2.1
 *
 * Напишите код для удаления дубликатов из несортированного связного списка.
 * + Дополнительно: Как вы будете решать задачу, если использовать временный буфер запрещено?
 */

use super::immutable_list::{List};

fn is_in_list(value: i32, list: &List) -> bool {
    match list {
        List::Nil => false,
        List::Node(value2, next) => value == *value2 || is_in_list(value, next),
    }
}

pub fn remove_duplicates(list: &List) -> List {
    match list {
        List::Nil => List::Nil,
        List::Node(value, next) => if is_in_list(*value, next) {
            remove_duplicates(next)
        } else {
            remove_duplicates(next).push(*value)
        }
    }
}

#[test]
fn remove_duplicates_with121341356_returns123456() {
    let list = List::from_vec(&vec![1, 2, 1, 3, 4, 1, 3, 5, 6]);
    let expected = List::from_vec(&vec![2, 4, 1, 3, 5, 6]);

    let actual = remove_duplicates(&list);

    assert_eq!(expected, actual);
}
