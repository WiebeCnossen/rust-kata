use std::borrow::Borrow;
use std::rc::Rc;

use super::list::List;

pub enum ConsNode<T> {
    Cons(Rc<T>, Rc<ConsNode<T>>),
    Nil,
}

use self::ConsNode::{Cons, Nil};

impl<T> ConsNode<T> {
    pub fn len(&self) -> usize {
        match self {
            &Cons(_, ref tail) => 1 + tail.len(),
            &Nil => 0,
        }
    }
}

fn unshift<T>(cons_node: &Rc<ConsNode<T>>, value: T) -> Rc<ConsNode<T>> {
    Rc::new(Cons(Rc::new(value), cons_node.clone()))
}

pub struct ConsList<T> {
    head: Rc<ConsNode<T>>,
}

impl<T: PartialEq> List<T> for ConsList<T> {
    type Node = Rc<ConsNode<T>>;

    fn empty() -> Self {
        ConsList { head: Rc::new(Nil) }
    }

    fn add(&self, value: T) -> Self {
        ConsList { head: unshift(&self.head, value) }
    }

    fn find(&self, value: &T) -> Option<&Rc<ConsNode<T>>> {
        let mut cons_node = &self.head;
        loop {
            match **cons_node {
                Cons(ref content, _) if *value == **content => return Some(cons_node),
                Cons(_, ref next) => cons_node = next,
                Nil => return None,
            }
        }
    }

    fn remove(&self, node: &Rc<ConsNode<T>>) -> Self {
        if let Cons(_, ref tail) = **node {
            let mut values: Vec<&Rc<T>> = vec![];

            let mut cons_node = &self.head;
            loop {
                if Rc::ptr_eq(cons_node, node) {
                    let head = values
                        .into_iter()
                        .fold(tail.clone(),
                              |head, &ref value| Rc::new(Cons(value.clone(), head)));
                    return ConsList { head };
                }

                match **cons_node {
                    Cons(ref value, ref next) => {
                        values.push(value);
                        cons_node = next;
                    }
                    Nil => panic!("Node is not in list"),
                }
            }
        } else {
            ConsList { head: self.head.clone() }
        }
    }

    fn collect(&self) -> Vec<&T> {
        let mut values = vec![];

        let mut node = &self.head;
        while let Cons(ref value, ref next) = **node {
            values.push(value.borrow());
            node = next
        }

        values.reverse();
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tests;

    #[test]
    fn new_is_empty() {
        let empty: Rc<ConsNode<u8>> = Rc::new(Nil);
        assert_eq!(0, empty.len());
    }

    #[test]
    fn with_one_added_is_one_long() {
        let empty: Rc<ConsNode<u8>> = Rc::new(Nil);
        let one = unshift(&empty, 0);
        assert_eq!(1, one.len());
    }

    #[test]
    fn add_two() {
        let empty: Rc<ConsNode<u8>> = Rc::new(Nil);
        let one = unshift(&empty, 0);
        let atwo = unshift(&one, 1);
        let btwo = unshift(&one, 2);
        assert_eq!(2, Rc::strong_count(&empty));
        assert_eq!(3, Rc::strong_count(&one));
        assert_eq!(2, atwo.len());
        assert_eq!(2, btwo.len());
    }

    #[test]
    fn test_empty() {
        tests::test_empty::<String, ConsList<String>>();
    }

    #[test]
    fn test_one() {
        tests::test_one::<String, ConsList<String>>(String::from("yo"));
    }

    #[test]
    fn test_two() {
        tests::test_two::<String, ConsList<String>>(String::from("yo"), String::from("lo"));
    }

    #[test]
    fn test_found() {
        tests::test_found::<String, ConsList<String>>(String::from("yo"), String::from("lo"));
    }

    #[test]
    fn test_not_found() {
        tests::test_not_found::<String, ConsList<String>>(String::from("yo"),
                                                          String::from("lo"),
                                                          String::from("no"));
    }

    #[test]
    fn test_remove_first() {
        tests::test_remove_first::<String, ConsList<String>>(String::from("yo"),
                                                             String::from("lo"));
    }

    #[test]
    fn test_remove_second() {
        tests::test_remove_second::<String, ConsList<String>>(String::from("yo"),
                                                              String::from("lo"));
    }

    #[test]
    fn test_remove_third() {
        tests::test_remove_third::<String, ConsList<String>>(String::from("yo"),
                                                             String::from("lo"),
                                                             String::from("go"));
    }
}
