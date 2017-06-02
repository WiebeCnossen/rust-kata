use std::fmt::Debug;

use super::list::List;

pub fn test_empty<T: PartialEq, L: List<T>>() {
    let list = L::empty();
    assert_eq!(0, list.collect().len());
}

pub fn test_one<T: PartialEq + Clone + Debug, L: List<T>>(v0: T) {
    let b0 = v0.clone();
    let list = L::empty().add(v0);
    let v = list.collect();
    assert_eq!(1, v.len());
    assert_eq!(b0, *v[0]);
}

pub fn test_two<T: PartialEq + Clone + Debug, L: List<T>>(v0: T, v1: T) {
    let b0 = v0.clone();
    let b1 = v1.clone();
    let list = L::empty().add(v0).add(v1);
    let v = list.collect();
    assert_eq!(2, v.len());
    assert_eq!(b0, *v[0]);
    assert_eq!(b1, *v[1]);
}

pub fn test_found<T: PartialEq + Clone, L: List<T>>(v0: T, v1: T) {
    let b0 = v0.clone();
    let list = L::empty().add(v0).add(v1);
    assert!(list.find(&b0).is_some());
}

pub fn test_not_found<T: PartialEq + Clone, L: List<T>>(v0: T, v1: T, x0: T) {
    let list = L::empty().add(v0).add(v1);
    assert!(list.find(&x0).is_none());
}

pub fn test_remove_first<T: PartialEq + Clone + Debug, L: List<T>>(v0: T, v1: T) {
    let b0 = v0.clone();
    let b1 = v1.clone();
    let list = L::empty().add(v0).add(v1);
    let found = list.find(&b0);
    assert!(found.is_some());
    let list = list.remove(found.unwrap());
    let v = list.collect();
    assert_eq!(1, v.len());
    assert_eq!(b1, *v[0]);
}

pub fn test_remove_second<T: PartialEq + Clone + Debug, L: List<T>>(v0: T, v1: T) {
    let b0 = v0.clone();
    let b1 = v1.clone();
    let list = L::empty().add(v0).add(v1);
    let found = list.find(&b1);
    assert!(found.is_some());
    let list = list.remove(found.unwrap());
    let v = list.collect();
    assert_eq!(1, v.len());
    assert_eq!(b0, *v[0]);
}

pub fn test_remove_third<T: PartialEq + Clone + Debug, L: List<T>>(v0: T, v1: T, v2: T) {
    let b0 = v0.clone();
    let b1 = v1.clone();
    let b2 = v2.clone();
    let list = L::empty().add(v0).add(v1).add(v2);
    let found = list.find(&b2);
    assert!(found.is_some());
    let list = list.remove(found.unwrap());
    let v = list.collect();
    assert_eq!(2, v.len());
    assert_eq!(b0, *v[0]);
    assert_eq!(b1, *v[1]);
}
