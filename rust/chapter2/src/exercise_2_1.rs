#[derive(PartialEq, Debug, Clone)]
pub struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

pub fn remove_duplicates(head: &mut Option<Box<ListNode>>) {
    let mut start = head;

    while start != Option::None {
        let value = start.unwrap().value;
        let next = start.unwrap().next;

        while next != Option::None {
            
        }
        let mut before_duplicate = find_before(&mut next, value);
        remove_after(before_duplicate);


    }
}

fn find_before(head: &mut Option<Box<ListNode>>, value: i32) -> &mut Option<Box<ListNode>> {
    if let Some(current_box) = head {
        if let Some(next_box) = &current_box.next {
            return if next_box.value == value {
                head
            } else {
                find_before(&mut current_box.next, value)
            }
        }
    }

    &mut Option::<Box<ListNode>>::None
}

#[test]
fn find_before_with_nil_returns_none() {
    let actual = find_before(&Option::None, 42);

    assert_eq!(&Option::None, actual);
}

#[test]
fn find_before_with_suitable_second_element_returns_head() {
    let head = Some(Box::new(ListNode {
        value: 41,
        next: Some(Box::new(ListNode {
            value: 42,
            next: None,
        }))
    }));

    let actual = find_before(&head, 42);

    assert_eq!(&head, actual);
}

#[test]
fn find_before_with_no_suitable_element_returns_none() {
    let head = Some(Box::new(ListNode {
        value: 43,
        next: Some(Box::new(ListNode {
            value: 44,
            next: None,
        }))
    }));

    let actual = find_before(&head, 42);

    assert_eq!(&Option::None, actual);
}

fn remove_after(node: &mut Option<Box<ListNode>>) {
    if let Some(node_box) = node {
        if let Some(next_box) = &node_box.next {
            node_box.next = next_box.next.clone()
        }
    }
}

#[test]
fn remove_after_with_head_removes_second_element() {
    let mut head = Some(Box::new(ListNode {
        value: 41,
        next: Some(Box::new(ListNode {
            value: 42,
            next: Some(Box::new(ListNode {
                value: 43,
                next: Option::None,
            })),
        })),
    }));

    remove_after(&mut head);

    assert_eq!(43, head.unwrap().next.unwrap().value);
}
