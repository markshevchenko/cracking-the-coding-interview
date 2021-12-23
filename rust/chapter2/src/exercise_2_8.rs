/*
 * Задача 2.8
 *
 * Для кольцевого связного списка реализуйте алгоритм, возвращающий начальный узел петли.
 *
 * Определение:
 * Кольцевой связный список — это связный список, в котором указатель следующего узла ссылается
 * на более ранний узел, образуя петлю.
 */

use std::mem;
use std::rc::Rc;

struct List {
    head: Option<Rc<Node>>,
}

fn are_nodes_equal(node1: &Option<Rc<Node>>, node2: &Option<Rc<Node>>) -> bool {
    match (node1, node2) {
        (None, None) =>
            true,
        (Some(node1), Some(node2)) =>
            node1.value == node2.value && are_nodes_equal(&node1.next, &node2.next),
        _ => false,
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        are_nodes_equal(&self.head, &other.head)
    }
}

#[derive(PartialEq)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let node = Rc::new(Node {
            value,
            next: mem::replace(&mut self.head, None),
        });

        self.head = Some(node);
    }

    pub fn from_vec(values: &Vec<i32>) -> Self {
        let mut result = List::new();

        for value in values.iter().rev() {
            result.push(*value);
        }

        result
    }

    pub fn try_make_cycle(&mut self, from_value: i32) -> bool {
        let last = if let Some(first) = &mut self.head {
            let mut last = first;
            loop {
                if let Some(next) = &mut last.next {
                    last = next;
                } else {
                    break;
                }
            }

            Some(last)
        } else {
            None
        };

        let from = if let Some(first) = &self.head {
            let mut from = first;
            loop {
                if from.value == from_value {
                    break;
                } else if let Some(next) = &from.next {
                    from = next;
                } else {
                    return false;
                }
            }

            Some(from)
        } else {
            None
        };

        if let Some(from) = from {
            if let Some(last) = last {
                last.next = Some(from.clone());

                return true;
            }
        }

        false
    }
}

#[test]
fn try_make_cycle_with_12345_3_returns_true() {
    let mut list = List::from_vec(&vec![1, 2, 3, 4, 5]);
    let actual = list.try_make_cycle(3);

    assert!(actual);
}