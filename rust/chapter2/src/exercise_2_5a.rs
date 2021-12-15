/*
 * Задача 2.5
 *
 * Два числа хранятся в виде связных списков, в которых каждый узел представляет один разряд. Все
 * цифры хранятся в обратном порядке, то есть младший разряд (единицы) хранится в начале списка.
 * Напишите функцию, которая суммирует два числа и возвращает результат в виде связного списка.
 *
 * Пример:
 * Ввод: (7->1->6) + (5->9->2), то есть 617 + 295
 * Вывод: 2->1->9, то есть 912
 *
 * 🡆 Дополнительно
 * Решите задачу, предполагая, что цифры записаны в прямом порядке.
 * Ввод: (6->1->7) + (2->9->5), то есть 617 + 295
 * Вывод: 9->1->2, то есть 912
 */

use super::mutable_list::{List, Node};

pub fn sum(a: List, b: List) -> List {
    match (a.head, b.head) {
        (None, None) => List { head: None },
        (Some(a_node), None) => List { head: Some(a_node) },
        (None, Some(b_node)) => List { head: Some(b_node) },
        (Some(a_node), Some(b_node)) => {
            let (carry, node) = sum_recursive(&a_node, &b_node);

            if carry == 0 {
                List { head: node }
            } else {
                List { head: Some(Box::new(Node { value: 1, next: node })) }
            }
        }
    }
}

fn sum_recursive(a: &Node, b: &Node) -> (i32, Option<Box<Node>>) {
    let (sum, node) = match (&a.next, &b.next) {
        (None, None) => {
            let sum = a.value + b.value;

            (sum, None)
        },
        (Some(a_next), None) => {
            let (carry, node) = sum_recursive(a_next, b);
            let sum = a.value + b.value + carry;

            (sum, node)
        },
        (None, Some(b_next)) => {
            let (carry, node) = sum_recursive(a, b_next);
            let sum = b.value + b.value + carry;

            (sum, node)
        },
        (Some(a_next), Some(b_next)) => {
            let (carry, node) = sum_recursive(a_next, b_next);
            let sum = a.value + b.value + carry;

            (sum, node)
        },
    };

    if sum >= 10 {
        (1, Some(Box::new(Node { value: sum - 10, next: node })))
    } else {
        (0, Some(Box::new(Node { value: sum, next: node })))
    }
}

#[test]
fn sum_with_617_295_returns_912() {
    let expected = List::from_vec(&vec![9, 1, 2]);
    let a = List::from_vec(&vec![6, 1, 7]);
    let b = List::from_vec(&vec![2, 9, 5]);

    let actual = sum(a, b);

    assert_eq!(expected, actual);
}