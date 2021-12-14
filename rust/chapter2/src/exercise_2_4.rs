/*
 * Задача 2.4
 *
 * Напишите код для разбиения связного списка вокруг значения x, так чтобы все узлы, меньшие x,
 * предшествовали узлам, большим или равным x. Если x содержится в списке, то значения x должны
 * следовать строго после элементов, меньших x. Элемент разбивки x может находиться где угодно в
 * "правой части", он не обязан располагаться между левой и правой частью.
 *
 * Пример:
 * Ввод: 3->5->8->5->10->2->1 [значение разбивки = 5]
 * Вывод: 3->1->2->10->5->5->8
 */

use super::immutable_list::{List};

pub fn partial_sort(list: &List, x: i32) -> List {
    let (mut before, after) = split(&list, x, List::new(), List::new());
    let mut result = after;

    while let List::Node(value, next) = before {
        result = result.push(value);
        before = *next;
    }

    result
}

#[test]
fn partial_sort_with3_5_8_5_10_2_1_and_x5_returns3_1_2_10_5_5_8() {
    let expected = List::from_vec(&vec![3, 2, 1, 10, 5, 8, 5]);

    let actual = partial_sort(&List::from_vec(&vec![3, 5, 8, 5, 10, 2, 1]), 5);

    assert_eq!(expected, actual);
}

fn split(list: &List, separator: i32, before: List, after: List) -> (List, List) {
    match list {
        List::Nil => (before, after),
        List::Node(value, next) => if *value < separator {
            split(&next, separator, before.push(*value), after)
        } else {
            split(&next, separator, before, after.push(*value))
        }
    }
}

#[test]
fn split_with123456_and_separator4_returns_321_654() {
    let (before, after) = split(
        &List::from_vec(&vec![1, 2, 3, 4, 5, 6]),
        4,
        List::new(),
        List::new());

    assert_eq!(List::from_vec(&vec![3, 2, 1]), before);
    assert_eq!(List::from_vec(&vec![6, 5, 4]), after);
}
