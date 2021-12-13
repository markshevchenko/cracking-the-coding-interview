#[derive(Debug)]
pub enum List {
    Nil,
    Node(i32, Box<List>),
}

impl List {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn from_vec(values: &Vec<i32>) -> Self {
        let mut result = List::new();

        for value in values.iter().rev() {
            result = result.push(*value);
        }

        result
    }

    pub fn push(self, value: i32) -> Self {
        List::Node(value, Box::new(self))
    }
}

#[test]
fn new_when_called_create_list_with_empty() {
    let actual = List::new();

    assert_eq!(List::Nil, actual);
}

#[test]
fn push_with_empty_and_1_returns_1() {
    let expected = List::Node(1, Box::new(List::Nil));

    let mut actual = List::new();
    actual = actual.push(1);

    assert_eq!(expected, actual);
}

#[test]
fn push_with_23_and_1_returns_123() {
    let expected =
        List::Node(1, Box::new(
            List::Node(2, Box::new(
                List::Node(3, Box::new(List::Nil))
            ))
        ));

    let mut actual = List::new();
    actual = actual.push(3);
    actual = actual.push(2);
    actual = actual.push(1);

    assert_eq!(expected, actual);
}

#[test]
fn from_vec_with_123_returns_123() {
    let expected =
        List::Node(1, Box::new(
            List::Node(2, Box::new(
                List::Node(3, Box::new(List::Nil))
            ))
        ));

    let actual = List::from_vec(&vec![1, 2, 3]);

    assert_eq!(expected, actual);
}

fn are_links_equal(list1: &List, list2: &List) -> bool {
    match (list1, list2) {
        (List::Nil, List::Nil) =>
            true,

        (List::Node(value1, next1), List::Node(value2, next2)) =>
            value1 == value2 && are_links_equal(next1, next2),

        _ =>
            false,
    }
}

#[test]
fn are_links_equal_with_none_none_returns_true() {
    assert!(are_links_equal(&List::Nil, &List::Nil));
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        are_links_equal(&self, &other)
    }
}

use std::fmt;

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut current = self;
        if let List::Node(value, next) = current {
            write!(f, "{}", value)?;

            current = next;
        }

        while let List::Node(value, next) = current {
            write!(f, ", {}", value)?;

            current = next;
        }

        write!(f, "]")
    }
}