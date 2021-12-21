#[derive(PartialEq, Debug)]
pub enum List {
    Nil,
    Cons(i32, std::rc::Rc<List>)
}

use std::fmt;

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut current = self;
        if let List::Cons(value, next) = current {
            write!(f, "{}", value)?;

            current = next;
        }

        while let List::Cons(value, next) = current {
            write!(f, ", {}", value)?;

            current = next;
        }

        write!(f, "]")
    }
}
