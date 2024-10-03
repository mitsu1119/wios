#![cfg_attr(test, no_std)]

use core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct LinkedList<T> {
    head: Option<NonNull<LinkedListItem<T>>>,
    last: Option<NonNull<LinkedListItem<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            last: None,
        }
    }

    pub fn push(&mut self, value: &mut LinkedListItem<T>) {
        let pt = unsafe { NonNull::new_unchecked(value as *mut LinkedListItem<T>) };
        if let Some(mut last) = self.last {
            unsafe { last.as_mut() }.next = Some(pt);
            self.last = Some(pt);
        } else {
            self.head = Some(pt);
            self.last = Some(pt);
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        if let Some(head) = self.head {
            let res = unsafe { head.as_ref() };
            self.head = res.next;
            if self.head.is_none() {
                self.last = None;
            }
            Some(res.deref())
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedListItem<T> {
    value: T,
    next: Option<NonNull<LinkedListItem<T>>>,
}

impl<T> LinkedListItem<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T> Deref for LinkedListItem<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for LinkedListItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod test {
    use super::{LinkedList, LinkedListItem};

    #[test]
    fn linked_list_test() {
        let mut list = LinkedList::<u32>::new();
        let mut val1 = LinkedListItem::new(1);
        let mut val2 = LinkedListItem::new(2);

        list.push(&mut val1);
        list.push(&mut val2);

        assert_eq!(list.pop(), Some(&1));
        assert_eq!(list.pop(), Some(&2));
        assert!(list.is_empty());

        let mut val3 = LinkedListItem::new(3);
        list.push(&mut val3);
        assert_eq!(list.pop(), Some(&3));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());

        list.push(&mut val1);
        list.push(&mut val3);
        assert_eq!(list.pop(), Some(&1));
        assert_eq!(list.pop(), Some(&3));
        assert!(list.is_empty());
    }
}
