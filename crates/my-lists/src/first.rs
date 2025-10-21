pub struct Stack<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // 将当前链表头节点替换为空节点，并返回原来的头节点
            // 此操作用于链表节点的移除或重置操作
            // 返回值：原来的链表头节点
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // 使用 std::mem::replace 原子性地替换链表头节点
        // 将当前的 head 节点替换为 Link::Empty，并返回原来的 head 值
        // 这种方式避免了直接移动所有权可能带来的借用检查问题
        match std::mem::replace(&mut self.head, Link::Empty) {

            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl <T> Drop for Stack<T> {
    fn drop(&mut self) {
        // 使用 std::mem::replace 原子地交换 head 和 Link::Empty，
        // 将原来的链表头节点保存到 cur_link 中，同时将 self.head 设置为空链表状态。
        // 这样可以在不破坏链表结构的情况下获取当前链表的头部引用。
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        // 遍历链表直到找到最后一个节点
        // 在遍历过程中，将当前节点的下一个链接替换为空链接
        // 这个操作会断开链表中除最后一个节点外的所有节点连接
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }

    }
}




// in first.rs
#[cfg(test)]
mod test {
    use crate::first::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        // Check empty list behaves right
        assert_eq!(stack.pop(), None);

        // Populate list
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut stack = Stack::new();
        for i in 0..100000 {
            stack.push(i);
        }
        drop(stack);
    }
}