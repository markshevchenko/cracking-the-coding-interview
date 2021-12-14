/*
 * Задача 2.3
 *
 * Реализуйте алгоритм, удаляющий узел из середины односвязного списка (то есть узел,
 * не являющийся ни начальным, ни конечным — не обязательно находящимся точно в середине).
 * Доступ предоставляется только к этому узлу.
 *
 * Пример:
 * Ввод: узел c из списка a->b->c->d->e->f
 * Вывод: ничего не возвращается, но новый список имеет вид: a->b->d->e->f
 */

use super::mutable_list::{List, Node};

pub fn remove_from_middle(list: &mut List, value: i32) {
    let mut node = find_mut_node_by_value(list, value);

    if let Some(next) = node.next.take() {
        node.value = next.value;
        node.next = next.next;
    }
}

fn find_mut_node_by_value(list: &mut List, value: i32) -> &mut Node {
    let mut next = &mut list.head;

    loop {
        if let Some(node) = next {
            if node.value == value {
                return node;
            }

            next = &mut node.next;
        }
    }
}

#[test]
fn remove_from_middle_with_123456_and_4_returns_12356() {
    let expected = List::from_vec(&vec![1, 2, 3, 5, 6]);
    let mut actual = List::from_vec(&vec![1, 2, 3, 4, 5, 6]);

    remove_from_middle(&mut actual, 4);

    assert_eq!(expected, actual);
}