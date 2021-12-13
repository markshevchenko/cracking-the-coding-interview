/*
 * Задача 2.2
 *
 * Реализуйте алгоритм для нахождения в односвязном списке k-го элемента с конца.
 */

use super::immutable_list::{List};

pub fn find_kth_from_end(list: &List, k: usize) -> Option<i32> {
    fn find_recursive(list: &List, k: usize) -> (i32, usize) {
        match list {
            List::Nil => (0, k + 1),
            List::Node(value, next) => {
                let result = find_recursive(next, k);
                match result.1 {
                    0 => result,
                    1 => (*value, 0),
                    _ => (0, result.1 - 1)
                }
            }
        }
    }

    match find_recursive(list, k) {
        (value, 0) => Some(value),
        _ => None,
    }
}

#[test]
fn find_kth_from_end_with_nil_returns_none() {
    let actual = find_kth_from_end(&List::Nil, 0);

    assert_eq!(None, actual);
}

#[test]
fn find_kth_from_end_when_list_contains_100_and_k_0_returns_100() {
    let list = List::from_vec(&vec![100]);

    let actual = find_kth_from_end(&list, 0);

    assert_eq!(Some(100), actual);
}

#[test]
fn find_kth_from_end_when_k_grater_than_length_of_list_returs_none() {
    let list = List::from_vec(&vec![1, 2, 3]);

    let actual = find_kth_from_end(&list, 4);

    assert_eq!(None, actual);
}

#[test]
fn find_kth_from_end_with_123456_and_k_2_returns_4() {
    let list = List::from_vec(&vec![1, 2, 3, 4, 5, 6]);

    let actual = find_kth_from_end(&list, 2);

    assert_eq!(Some(4), actual);
}