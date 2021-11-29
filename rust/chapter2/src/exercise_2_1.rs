#[derive(Debug)]
enum Node {
    Nil,
    List(i32, Box<Node>),
}

#[derive(Debug)]
struct List {
    root: Node,
}

fn build_list(items: Vec<i32>) -> List {
    let mut head = Node::Nil;

    for item in items.iter().rev() {
        head = Node::List(*item, Box::new(head))
    }

    List { root: head }
}

fn are_lists_equal(node1: &Node, node2: &Node) -> bool {
    match (node1, node2) {
        (Node::Nil, Node::Nil) =>
            true,
        (Node::List(a, tail1), Node::List(b, tail2)) =>
            a == b && are_lists_equal(tail1.as_ref(), tail2.as_ref()),
        (_, _) => false
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        are_lists_equal(&self.root, &other.root)
    }
}

#[test]
fn build_list_with_empty_vector_returns_nil() {
    let list = build_list(vec![]);

    assert_eq!(List { root: Node::Nil }, list);
}

#[test]
fn build_list_with_123_builds_list_123() {
    let expected = List { root: Node::List(1, Box::new(
        Node::List(2, Box::new(
            Node::List(3, Box::new(Node::Nil))
        ))
    ))};

    let list = build_list(vec![1, 2, 3]);

    assert_eq!(expected, list);
}

fn find_before_mut(item: i32, node: &mut Node) -> &mut Node {
    if let Node::List(_, tail) = node {
        if let Node::List(next_item, next_tail) = tail.as_mut() {
            if item == *next_item {
                return node;
            } else {
                return find_before_mut(item, tail.as_mut())
            }
        }
    }

    &mut Node::Nil
}



fn remove_item(item: i32, node: &mut Node) {
    match node {
        Node::Nil => (),
        List
    }
}

fn remove_duplicates(list: &mut Node) {
    match node {
        List(value, tail) => remove_duplicates(value, tail.as_mut_ref())
    }

    if node == Node::Nil {
        return;
    }



}

// pub fn remove_duplicates(head: &mut Option<Box<ListNode>>) {
//     let mut start = head;
//
//     while start != Option::None {
//         let value = start.unwrap().value;
//         let next = start.unwrap().next;
//
//         while next != Option::None {
//
//         }
//         let mut before_duplicate = find_before(&mut next, value);
//         remove_after(before_duplicate);
//
//
//     }
// }
//
// fn find_before(head: &mut Option<Box<ListNode>>, value: i32) -> &mut Option<Box<ListNode>> {
//     if let Some(current_box) = head {
//         if let Some(next_box) = &current_box.next {
//             return if next_box.value == value {
//                 head
//             } else {
//                 find_before(&mut current_box.next, value)
//             }
//         }
//     }
//
//     &mut Option::<Box<ListNode>>::None
// }
//
// #[test]
// fn find_before_with_nil_returns_none() {
//     let actual = find_before(&Option::None, 42);
//
//     assert_eq!(&Option::None, actual);
// }
//
// #[test]
// fn find_before_with_suitable_second_element_returns_head() {
//     let head = Some(Box::new(ListNode {
//         value: 41,
//         next: Some(Box::new(ListNode {
//             value: 42,
//             next: None,
//         }))
//     }));
//
//     let actual = find_before(&head, 42);
//
//     assert_eq!(&head, actual);
// }
//
// #[test]
// fn find_before_with_no_suitable_element_returns_none() {
//     let head = Some(Box::new(ListNode {
//         value: 43,
//         next: Some(Box::new(ListNode {
//             value: 44,
//             next: None,
//         }))
//     }));
//
//     let actual = find_before(&head, 42);
//
//     assert_eq!(&Option::None, actual);
// }
//
// fn remove_after(node: &mut Option<Box<ListNode>>) {
//     if let Some(node_box) = node {
//         if let Some(next_box) = &node_box.next {
//             node_box.next = next_box.next.clone()
//         }
//     }
// }
//
// #[test]
// fn remove_after_with_head_removes_second_element() {
//     let mut head = Some(Box::new(ListNode {
//         value: 41,
//         next: Some(Box::new(ListNode {
//             value: 42,
//             next: Some(Box::new(ListNode {
//                 value: 43,
//                 next: Option::None,
//             })),
//         })),
//     }));
//
//     remove_after(&mut head);
//
//     assert_eq!(43, head.unwrap().next.unwrap().value);
// }
