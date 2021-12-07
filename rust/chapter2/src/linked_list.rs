#[derive(Debug)]
pub struct List {
    pub(crate) head: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Node {
    pub(crate) value: i32,
    pub(crate) next: Option<Box<Node>>,
}

use std::mem;

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }

    pub fn from_vec(values: &Vec<i32>) -> Self {
        let mut result = List::new();

        for value in values.iter().rev() {
            result.push(*value);
        }

        result
    }
}

#[test]
fn new_when_called_create_list_with_empty() {
    let actual = List::new();
    let expected = List { head: None };

    assert_eq!(expected, actual);
}

#[test]
fn push_with_empty_and_1_returns_1() {
    let mut actual = List::new();
    actual.push(1);
    let expected = List { head: Some(Box::new(Node {
        value: 1,
        next: None,
    }))};

    assert_eq!(expected, actual);
}

#[test]
fn push_with_23_and_1_returns_123() {
    let expected = List { head: Some(Box::new(Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: Some(Box::new(Node {
                value: 3,
                next: None,
            })),
        })),
    }))};
    let mut actual = List::new();
    actual.push(3);
    actual.push(2);

    actual.push(1);

    assert_eq!(expected, actual);
}

#[test]
fn from_vec_with_123_returns_123() {
    let expected = List { head: Some(Box::new(Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: Some(Box::new(Node {
                value: 3,
                next: None,
            })),
        })),
    }))};
    let actual = List::from_vec(&vec![1, 2, 3]);

    assert_eq!(expected, actual);
}

fn are_links_equal(node1: &Option<Box<Node>>, node2: &Option<Box<Node>>) -> bool {
    match (node1, node2) {
        (None, None) =>
            true,

        (Some(node1), Some(node2)) =>
            node1.value == node2.value && are_links_equal(&node1.next, &node2.next),

        _ =>
            false,
    }
}

#[test]
fn are_links_equal_with_none_none_returns_true() {
    assert!(are_links_equal(&None, &None));
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        are_links_equal(&self.head, &other.head)
    }
}

use std::fmt;

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut current = &self.head;
        if let Some(current_node) = current {
            write!(f, "{}", current_node.value)?;

            current = &current_node.next;
        }

        while let Some(current_node) = current {
            write!(f, ", {}", current_node.value)?;

            current = &current_node.next;
        }

        write!(f, "]")
    }
}