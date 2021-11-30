// fn find_before(item: i32, List: list) -> Node {
//     match node {
//         Node::List(_, tail) => match tail.as_ref() {
//             Node::List(next_item, next_tail) => if item == *next_item {
//                 return node;
//             } else {
//                 return find_before(item, next_tail.as_ref());
//             },
//             _ => &Node::Nil,
//         },
//         _ => &Node::Nil,
//     }
// }
//
// fn remove_after(node: &mut Node) {
//     match node {
//         Node::List(_, tail) => match tail.as_ref() {
//             Node::List(_, next_tail) =>
//         }
//     }
// }
//
// #[test]
// fn find_before_when_2nd_item_found_returns_1st_item() {
//     let list = build_list(vec![1, 2, 3]);
//
//     let node = find_before(2, &list.root);
//
//     if let Node::List(actual, _) = node {
//         assert_eq!(1, *actual);
//     } else {
//         assert!(false);
//     }
// }

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
