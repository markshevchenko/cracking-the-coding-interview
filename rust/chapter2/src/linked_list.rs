pub struct List {
    head: Link,
}

enum Link {
    Empty,
    Tail(Box<Node>),
}

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

    pub fn new1(value1: i32) -> Self {
        let mut result = List::new();
        result.push(value1);

        result
    }

    pub fn new2(value1: i32, value2: i32) -> Self {
        let mut result = List::new();
        result.push(value1);
        result.push(value2);

        result
    }

    pub fn new3(value1: i32, value2: i32, value3: i32) -> Self {
        let mut result = List::new();
        result.push(value1);
        result.push(value2);
        result.push(value3);

        result
    }
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