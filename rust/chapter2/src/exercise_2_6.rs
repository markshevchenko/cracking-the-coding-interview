/*
 * Задача 2.6
 *
 * Реализуйте функцию, проверяющую, является ли связный список палиндромом.
 */

use super::immutable_list::{List};

pub fn is_palindrome(list: &List) -> bool {
    let inversion = reverse(list, List::Nil);

    *list == inversion
}

#[test]
fn is_palindrome_with_1234321_returns_true() {
    let list = List::from_vec(&vec![1, 2, 3, 4, 3, 2, 1]);

    assert!(is_palindrome(&list));
}

#[test]
fn is_palindrome_with_1234312_returns_false() {
    let list = List::from_vec(&vec![1, 2, 3, 4, 3, 1, 2]);

    assert!(!is_palindrome(&list));
}

fn reverse(list: &List, accumulator: List) -> List {
    if let List::Node(value, next) = list {
        reverse(next, accumulator.push(*value))
    } else {
        accumulator
    }
}

#[test]
fn reverse_with_123456_nil_returns_654321() {
    let expected = List::from_vec(&vec![6, 5, 4, 3, 2, 1]);

    let actual = reverse(&List::from_vec(&vec![1, 2, 3, 4, 5, 6]), List::Nil);

    assert_eq!(expected, actual);
}

#[test]
fn reverse_with_nil_nil_returns_nil() {
    let expected = List::Nil;

    let actual = reverse(&List::Nil, List::Nil);

    assert_eq!(expected, actual);
}