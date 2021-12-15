/*
 * Задача 2.5
 *
 * 🡆 Два числа хранятся в виде связных списков, в которых каждый узел представляет один разряд. Все
 * цифры хранятся в обратном порядке, то есть младший разряд (единицы) хранится в начале списка.
 * Напишите функцию, которая суммирует два числа и возвращает результат в виде связного списка.
 *
 * Пример:
 * Ввод: (7->1->6) + (5->9->2), то есть 617 + 295
 * Вывод: 2->1->9, то есть 912
 *
 * Дополнительно
 * Решите задачу, предполагая, что цифры записаны в прямом порядке.
 * Ввод: (6->1->7) + (2->9->5), то есть 617 + 295
 * Вывод: 9->1->2, то есть 912
 */

use super::immutable_list::{List};

pub fn sum(a: &List, b: &List) -> List {
    fn sum_with_carry(a: &List, b: &List, carry: i32) -> List {
        match (a, b) {
            (List::Nil, List::Nil) => if carry == 1 {
                List::Node(1, Box::new(List::Nil))
            } else {
                List::Nil
            },
            (List::Node(a_value, a_next), List::Nil) => {
                let sum = a_value + carry;
                if sum >= 10 {
                    sum_with_carry(a_next, &List::Nil, 1).push(sum - 10)
                } else {
                    sum_with_carry(a_next, &List::Nil, 0).push(sum)
                }
            },
            (List::Nil, List::Node(b_value, b_next)) => {
                let sum = b_value + carry;
                if sum >= 10 {
                    sum_with_carry(&List::Nil, b_next, 1).push(sum - 10)
                } else {
                    sum_with_carry(&List::Nil, b_next, 0).push(sum)
                }
            },
            (List::Node(a_value, a_next), List::Node(b_value, b_next)) => {
                let sum = a_value + b_value + carry;
                if sum >= 10 {
                    sum_with_carry(a_next, b_next, 1).push(sum - 10)
                } else {
                    sum_with_carry(a_next, b_next, 0).push(sum)
                }
            },
        }
    }

    sum_with_carry(a, b, 0)
}

#[test]
fn sum_with_716_592_returns_219() {
    let expected = List::from_vec(&vec![2, 1, 9]);
    let a = List::from_vec(&vec![7, 1, 6]);
    let b = List::from_vec(&vec![5, 9, 2]);

    let actual = sum(&a, &b);

    assert_eq!(expected, actual);
}