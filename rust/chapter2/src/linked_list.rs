#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    Tail(Box<Node>),
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Link,
}

use std::mem;

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::Tail(new_node);
    }

    pub fn new_vec(values: &Vec<i32>) -> Self {
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
    let expected = List { head: Link::Empty };

    assert_eq!(expected, actual);
}

#[test]
fn push_with_empty_and_1_returns_1() {
    let mut actual = List::new();
    actual.push(1);
    let expected = List { head: Link::Tail(Box::new(Node {
        value: 1,
        next: Link::Empty,
    }))};

    assert_eq!(expected, actual);
}

#[test]
fn push_with_23_and_1_returns_123() {
    let expected = List { head: Link::Tail(Box::new(Node {
        value: 1,
        next: Link::Tail(Box::new(Node {
            value: 2,
            next: Link::Tail(Box::new(Node {
                value: 3,
                next: Link::Empty,
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
fn new_vec_with_123_returns_123() {
    let expected = List { head: Link::Tail(Box::new(Node {
        value: 1,
        next: Link::Tail(Box::new(Node {
            value: 2,
            next: Link::Tail(Box::new(Node {
                value: 3,
                next: Link::Empty,
            })),
        })),
    }))};
    let actual = List::new_vec(&vec![1, 2, 3]);

    assert_eq!(expected, actual);
}

fn are_links_equal(link1: &Link, link2: &Link) -> bool {
    match (link1, link2) {
        (Link::Empty, Link::Empty) =>
            true,

        (Link::Tail(node1), Link::Tail(node2)) =>
            node1.value == node2.value && are_links_equal(&node1.next, &node2.next),

        _ =>
            false,
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        are_links_equal(&self.head, &other.head)
    }
}
