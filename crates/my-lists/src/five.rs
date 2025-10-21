use std::{mem, ptr};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    real_next: Option<Box<Node<T>>>,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            // Box::into_raw 是 Rust 标准库中的一个重要函数，其作用如下：
            // 主要功能
            // 1. 转换所有权：
            //      * 将 Box<T> 的所有权转换为裸指针 *mut T
            //      * 不会释放 Box<T> 管理的内存
            // 2. 内存管理转移：
            //      * 将堆上分配的内存的管理责任转移给调用者
            //      * 调用者需要负责后续的内存释放
            let new_tail = Box::into_raw(Box::new(Node {
                elem: elem,
                real_next: None,
                next: ptr::null_mut(),
            }));

            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
            }
            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                // Box::from_raw 是 Rust 标准库中与 Box::into_raw 配对使用的重要函数，其作用如下：
                // 主要功能
                // 1. 指针转Box：
                //      * 将裸指针 *mut T 转换回 Box<T>
                //      * 重新获得Rust的自动内存管理能力
                // 2. 内存管理回收：
                //      * 重新接管之前通过 Box::into_raw 转移出去的内存管理权
                //      * 当返回的 Box<T> 被丢弃时，会自动释放对应的堆内存
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}


pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}


pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
