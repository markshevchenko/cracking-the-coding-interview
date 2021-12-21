/*
 * Задача 2.7
 *
 * Проверьте, пересекаются ли два заданных связных списка. Верните узел пересечения. Учтите,
 * что пересечение определяется ссылкой, а не значением. Иначе говоря, если k-й узел первого
 * связного списка точно совпадает (по ссылке) с j-м узлом второго связного списка, то списки
 * считаются пересекающимися.
 */

use std::rc::Rc;
use super::complex_list::{List};

fn is_in_list(node: &Rc<List>, list: &Rc<List>) -> bool {
    if Rc::<List>::ptr_eq(node, list) {
        true
    } else if let List::Cons(_, next) = list.as_ref() {
        is_in_list(node, next)
    } else {
        false
    }
}

#[test]
fn is_in_list_with_node_in_list_returns_true() {
    let node1 = Rc::new(List::Cons(1, Rc::new(List::Nil)));
    let node2 = Rc::new(List::Cons(2, node1.clone()));
    let node3 = Rc::new(List::Cons(3, node2.clone()));

    let list = node3.clone();

    assert!(is_in_list(&node2, &list));
}

#[test]
fn is_in_list_with_node_not_in_list_returns_false() {
    let node1 = Rc::new(List::Cons(1, Rc::new(List::Nil)));
    let node2 = Rc::new(List::Cons(2, node1.clone()));
    let node3 = Rc::new(List::Cons(3, node2.clone()));

    let node4 = Rc::new(List::Cons(4, Rc::new(List::Nil)));
    let list = node3.clone();

    assert!(!is_in_list(&node4, &list));
}

pub fn has_common_node(list1: &Rc<List>, list2: &Rc<List>) -> bool {
    if is_in_list(list1, list2) {
        true
    } else if let List::Cons(_, next) = list1.as_ref() {
        has_common_node(next, list2)
    } else {
        false
    }
}

#[test]
fn has_common_node_with_common_tail_returns_true() {
    let tail = Rc::new(
        List::Cons(3, Rc::new(
            List::Cons(2, Rc::new(
                List::Cons(1, Rc::new(List::Nil))
            ))
        ))
    );

    let list1 = Rc::new(
        List::Cons(5, Rc::new(
            List::Cons(4, tail.clone())
        ))
    );

    let list2 = Rc::new(
        List::Cons(5, Rc::new(
            List::Cons(4, tail)
        ))
    );

    assert!(has_common_node(&list1, &list2));
}

#[test]
fn has_common_node_with_different_lists_returns_false() {
    let list1 = Rc::new(
        List::Cons(9, Rc::new(
            List::Cons(8, Rc::new(
                List::Cons(7, Rc::new(List::Nil))
            ))
        ))
    );

    let list2 = Rc::new(
        List::Cons(9, Rc::new(
            List::Cons(8, Rc::new(
                List::Cons(7, Rc::new(List::Nil))
            ))
        ))
    );

    assert!(!has_common_node(&list1, &list2));
}