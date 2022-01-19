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
use std::cell::Cell;

struct List {
    head: Option<Rc<Cell<Node>>>,
}

struct Node {
    value: i32,
    next: Option<Rc<Cell<Node>>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let node = Rc::new(
            Cell::new(Node {
                value,
                next: mem::replace(&mut self.head, None),
            })
        );

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
        let last = if let Some(first) = &self.head {
            let mut last = first.as_ref();
            loop {
                if let Some(last_node) = last.into_inner().next {
                    last = last_node.as_ref();
                } else {
                    break;
                }
            }

            Some(last)
        } else {
            None
        };

        let from = if let Some(first) = &self.head {
            let mut from = first.into_inner();
            loop {
                if from.value == from_value {
                    break;
                } else if let Some(next) = from.next {
                    from = next.into_inner();
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
                last.set(Node {
                    value: last.into_inner().value,
                    next: Some(Rc::new(Cell::new(from))),
                });

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